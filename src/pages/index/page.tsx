import React from "react";
import { Button } from "../../ui/button";
import { invoke } from "@tauri-apps/api";
import { readDir, BaseDirectory, FileEntry, createDir } from "@tauri-apps/plugin-fs";
import { FileIcon, Folder } from "lucide-react";

export default function Index() {
    const [files, setFiles] = React.useState<FileEntry[]>([]);
    const [path, setPath] = React.useState<string>("");

    React.useEffect(() => {
        async function findFiles() {
            await readDir(`Horizon/${path}`, { dir: BaseDirectory.Audio, recursive: true }).then((res) => {
                setFiles(res);
            }).catch(async (err) => {
                await createDir(`Horizon`, { dir: BaseDirectory.Audio }).then(() => {
                    setPath("");
                }).catch(() => {
                    console.error("Couldn't create Horizon folder.");
                });
            });
        }

        findFiles();
    }, [path]);

    function handleFilePlay(path: string) {
        invoke("play_file", { path }).then(() => {
            return true;
        }).catch(() => {
            return false;
        });
    }

    return (
        <>
            <main className="flex flex-col items-center p-8">
                <div className="flex flex-col items-center gap-8">
                    <h1 className='text-3xl font-black'>
                        Welcome!
                    </h1>
                    <p className="text-center text-primary-500">
                        Horizon is looking for files in the "/Horizon" folder, located at your system's default music directory.
                    </p>
                    <div className="flex flex-col items-center gap-2">
                        {path.length > 0 && (
                            <Button className="flex flex-row items-center" onClick={() => {
                                setPath(path.split("/").slice(0, -1).join("/"));
                            }}>
                                <Folder className="w-4 h-4" />
                                <span className="ml-2">Go back</span>
                            </Button>
                        )}
                        {files.map((file) => {
                            return (
                                <Button className="flex flex-row items-center" onClick={() => {
                                    if (file.children) {
                                        setPath(file.path.split("Horizon/")[1]);
                                    } else {
                                        handleFilePlay(file.path);
                                    }
                                }}>
                                    {file.children ? (
                                        <Folder className="w-4 h-4" />
                                    ) : (
                                        <FileIcon className="w-4 h-4" />
                                    )}
                                    <span className="ml-2">{file.name}</span>
                                </Button>
                            )
                        })}
                    </div>
                </div>
            </main>
        </>
    )
}