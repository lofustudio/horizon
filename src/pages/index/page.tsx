import React from "react";
import { Search, File, Folder } from "lucide-react";
import { Input } from "../../ui/input";
import { Button } from "../../ui/button";
import { invoke } from "@tauri-apps/api";
import { audioDir } from "@tauri-apps/api/path";

export default function Index() {
    const [audioPath, setAudioPath] = React.useState("");
    const [path, setPath] = React.useState("");

    React.useEffect(() => {
        audioDir().then((res) => {
            console.log(res)
            setAudioPath(res);
        }).catch((e) => console.error(e));
    });

    function handleFilePlay() {
        invoke("play_file", {
            path: `${audioPath}/${path}`
        }).then(() => {
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
                        Horzion uses your system music folder as it's path root. Specify the path to the file you want to play below.
                    </p>
                    <div className="flex flex-row items-center gap-2">
                        <div className="flex flex-row items-center justify-center w-full px-2 border rounded-md max-h-9 dark:bg-primary-700/50 border-black/10 dark:border-white/10">
                            <div className="flex flex-row items-center gap-2">
                                <Folder className="w-4 h-4 shrink-0" />
                                <p className="text-sm text-primary-500">
                                    {audioPath}\
                                </p>
                            </div>
                            <Input className="w-full p-0 bg-transparent border-none dark:bg-transparent h-9" placeholder="test.mp3" value={path} onChange={(e) => setPath(e.target.value)} />
                        </div>
                        <Button size="icon" onClick={() => {
                            handleFilePlay();
                        }}>
                            <File className="w-4 h-4" />
                        </Button>
                    </div>
                </div>
            </main>
        </>
    )
}