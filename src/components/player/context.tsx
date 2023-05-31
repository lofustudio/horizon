import { invoke } from "@tauri-apps/api";
import { emit, listen } from "@tauri-apps/api/event";
import React from "react";

export interface PlayerContext {
    tracks: HorizonTrack[];
    currentTrack: HorizonTrack | null;
    playing: boolean;
    paused: boolean;

    playFile: (track: HorizonTrack) => Promise<void>;
    togglePause: () => Promise<void>;
}

export const PlayerContext = React.createContext<PlayerContext>({
    tracks: [],
    currentTrack: null,
    playing: false,
    paused: false,

    playFile: async () => { },
    togglePause: async () => { },
});

export const PlayerProvider = ({ children }: { children: React.ReactNode }) => {
    const [mounted, setMounted] = React.useState<boolean>(false);
    const [currentTrack, setCurrentTrack] = React.useState<HorizonTrack | null>(null);
    const [tracks, setTracks] = React.useState<HorizonTrack[]>([]);

    const [playing, setPlaying] = React.useState<boolean>(false);
    const [paused, setPaused] = React.useState<boolean>(false);

    const playFile = async (track: HorizonTrack) => {
        await invoke("play_file", { path: track.path }).then((res: any) => {
            setPlaying(true);
            setCurrentTrack(track);
        }).catch((err) => {
            console.error(err);
        });
    };

    const togglePause = async () => {
        invoke("pause").then((res) => {
            // TODO: Get paused state from backend
            setPaused(!paused);
            setPlaying(!playing);
        }).catch((err) => {
            console.error(err);
        });
    }

    React.useEffect(() => {
        const unlisten = listen<HorizonTrack[]>("tracks", (e) => {
            console.log("Mounted payload: ", e.payload);
            setTracks(e.payload);
            setMounted(true);
        });

        emit("mounted");

        return () => {
            unlisten.then(f => f());
        }
    }, []);

    const value = React.useMemo(() => ({
        tracks,
        currentTrack,
        playing,
        paused,
        playFile,
        togglePause
    }), [
        tracks,
        currentTrack,
        playing,
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
