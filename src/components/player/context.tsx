import { invoke } from "@tauri-apps/api";
import {
    BaseDirectory,
    FileEntry,
    createDir,
    readDir
} from "@tauri-apps/plugin-fs";
import React from "react";

export interface PlayerContext {
    files: FileEntry[];
    tracks: HorizonTrack[];
    currentTrack: HorizonTrack | null;

    playing: boolean;
    paused: boolean;

    playFile: (path: string) => Promise<void>;
    togglePause: () => Promise<void>;
}

export const PlayerContext = React.createContext<PlayerContext>({
    files: [],
    tracks: [],
    currentTrack: null,

    playing: false,
    paused: false,

    playFile: async () => { },
    togglePause: async () => { },
});

export const PlayerProvider = ({ children }: { children: React.ReactNode }) => {
    const [mounted, setMounted] = React.useState<boolean>(false);

    const [files, setFiles] = React.useState<FileEntry[]>([]);
    const [tracks, setTracks] = React.useState<HorizonTrack[]>([]);
    const [currentTrack, setCurrentTrack] = React.useState<HorizonTrack | null>(null);

    const [playing, setPlaying] = React.useState<boolean>(false);
    const [paused, setPaused] = React.useState<boolean>(false);

    const playFile = async (path: string) => {
        await invoke("play_file", { path }).then((res: any) => {
            setPlaying(true);
            setCurrentTrack({ ...res, path });
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
        setMounted(true);

        async function findFiles() {
            await readDir(`Horizon`, { dir: BaseDirectory.Audio, recursive: true }).then((res) => {
                res.forEach(async (file) => {
                    if (file.name?.endsWith(".mp3") || file.name?.endsWith(".wav") || file.name?.endsWith(".ogg") || file.name?.endsWith(".flac") || file.children) {
                        setFiles((files) => [...files, { ...file }]);
                    }
                });
            }).catch(async (err) => {
                await createDir(`Horizon`, { dir: BaseDirectory.Audio }).then(() => {
                    // Trigger a re-run
                    setMounted(true);
                }).catch(() => {
                    console.error("Couldn't create Horizon folder.");
                });
            });
        }

        findFiles();
    }, []);

    const value = React.useMemo(() => ({
        files,
        tracks,
        currentTrack,
        playing,
        paused,
        playFile,
        togglePause
    }), [
        files,
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
