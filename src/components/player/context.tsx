import { invoke } from "@tauri-apps/api";
import { BaseDirectory, FileEntry, createDir, readDir } from "@tauri-apps/plugin-fs";
import React from "react";

export interface PlayerContext {
    files: FileEntry[];

    playing: boolean;
    currentFile: string;

    playFile: (path: string) => Promise<void>;
}

export const PlayerContext = React.createContext<PlayerContext>({
    files: [],

    playing: false,
    currentFile: "",

    playFile: async () => { },
});

export const PlayerProvider = ({ children }: { children: React.ReactNode }) => {
    const [mounted, setMounted] = React.useState<boolean>(false);

    const [files, setFiles] = React.useState<FileEntry[]>([]);

    const [playing, setPlaying] = React.useState<boolean>(false);
    const [currentFile, setCurrentFile] = React.useState<string>("");

    const playFile = async (path: string) => {
        invoke("play_file", { path }).then((res) => {
            setPlaying(true);
            setCurrentFile(path);
        }).catch((err) => {
            console.error(err);
        });
    };

    React.useEffect(() => {
        setMounted(true);

        async function findFiles() {
            await readDir(`Horizon`, { dir: BaseDirectory.Audio, recursive: true }).then((res) => {
                setFiles(res);
            }).catch(async (err) => {
                await createDir(`Horizon`, { dir: BaseDirectory.Audio }).catch(() => {
                    console.error("Couldn't create Horizon folder.");
                });
            });
        }

        findFiles();
    }, []);

    const value = React.useMemo(() => ({
        files,
        playing,
        currentFile,
        playFile,
    }), []);

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
