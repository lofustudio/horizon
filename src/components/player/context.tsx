import { invoke } from "@tauri-apps/api";
import { emit, listen } from "@tauri-apps/api/event";
import React from "react";
import { Status } from "../../types/Status";

export interface PlayerContext {
    tracks: HorizonTrack[];
    currentTrack: HorizonTrack | null;
    paused: boolean;

    playFile: (track: HorizonTrack) => Promise<void>;
    togglePause: () => Promise<void>;
}

export const PlayerContext = React.createContext<PlayerContext>({
    tracks: [],
    currentTrack: null,
    paused: false,

    playFile: async () => { },
    togglePause: async () => { },
});

export const PlayerProvider = ({ children }: { children: React.ReactNode }) => {
    const [mounted, setMounted] = React.useState<boolean>(false);
    const [currentTrack, setCurrentTrack] = React.useState<HorizonTrack | null>(null);
    const [tracks, setTracks] = React.useState<HorizonTrack[]>([]);

    const [paused, setPaused] = React.useState<boolean>(false);

    const playFile = async (track: HorizonTrack) => {
        await invoke("play_file", { path: track.path }).then((res: any) => {
            setCurrentTrack(track);
        }).catch((err) => {
            console.error(err);
        });
    };

    const togglePause = async () => {
        invoke<boolean>("toggle_pause").then((res) => {
            setPaused(res);
        }).catch((err) => {
            console.error(err);
        });
    }

    const status = async () => {
        invoke<Status>("status").then((res) => {
            console.log(res)
            setPaused(res.paused)
        }).catch((err) => {
            console.error(err);
        });
    }

    React.useEffect(() => {
        const unlistenTracks = listen<HorizonTrack[]>("tracks", (e) => {
            console.log("Mounted payload: ", e.payload);
            setTracks(e.payload);
            setMounted(true);
        });

        emit("mounted");

        status();

        return () => {
            unlistenTracks.then(f => f());
        }
    }, []);

    const value = React.useMemo(() => ({
        tracks,
        currentTrack,
        paused,
        playFile,
        togglePause
    }), [
        tracks,
        currentTrack,
        paused,
        playFile,
        togglePause
    ]);

    return (
        <>
            <PlayerContext.Provider value={value}>
                {mounted && children}
            </PlayerContext.Provider>
        </>
    )
}

export const usePlayer = () => {
    const context = React.useContext(PlayerContext);

    if (context === undefined) {
        throw new Error('usePlayer must be used within a PlayerProvider')
    }

    return context;
}
