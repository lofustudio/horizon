import React from "react";
import { usePlayer } from "../../components/player/context";
import { FileEntry } from "@tauri-apps/plugin-fs";

function Folder({ file }: { file: FileEntry }) {
    const { playFile, togglePause, paused, playing, files } = usePlayer();
    const [expanded, setExpanded] = React.useState(false);

    return (
        <>
            <button key={file.path} className="flex flex-row items-center w-full gap-4 p-2 transition-colors border rounded-sm dark:border-white/10 border-black/10 hover:dark:bg-primary-700/50" onClick={() => setExpanded(!expanded)}>
                <svg data-expanded={expanded} className="-rotate-90 data-[expanded=true]:rotate-0 transition-all" stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 24 24" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><path fill="none" d="M0 0h24v24H0V0z"></path><path d="M7.41 8.59L12 13.17l4.59-4.58L18 10l-6 6-6-6 1.41-1.41z"></path></svg>
                {/* <img src="https://lastfm.freetls.fastly.net/i/u/300x300/79d173d6a9926477816cea31d409a2d6.png" className="w-12 h-12" /> */}
                <div className="flex flex-col items-start pr-2 py-">
                    <span className="text-sm font-bold">{file.name}</span>
                    <span className="text-xs">{file.children ? file.children.length + " songs" : "Song"}</span>
                </div>
            </button>
            {expanded && (
                <>
                    {file.children?.map((child) => {
                        return (
                            <>
                                <File file={child} inline />
                            </>
                        )
                    })}
                </>
            )}
        </>
    )
}

function File({ file, inline = false }: { file: FileEntry, inline?: boolean }) {
    const { playFile } = usePlayer();

    return (
        <>
            <button key={file.path} data-inline={inline} className="flex flex-row items-center w-full gap-4 transition-colors rounded-sm hover:dark:bg-primary-700/50 data-[inline=true]:ml-16" onClick={() => {
                playFile(file.path)
            }}>
                <img src="https://via.placeholder.com/48x48/eee?text=Album" className="w-12 h-12" />
                <div className="flex flex-col items-start pr-2 py-">
                    <span className="text-sm font-bold">{file.name}</span>
                    <span className="text-xs">{file.path.split("\\")[6]}</span>
                </div>
            </button>
        </>
    )
}

export default function Index() {
    const { paused, playing, files } = usePlayer();

    console.log("Paused: " + paused);
    console.log("Playing: " + playing);

    console.log("Files: " + JSON.stringify(files));

    return (
        <>
            <main className="flex flex-col items-start w-full gap-8 p-8">
                <h1 className="text-3xl font-bold">
                    Your Library
                </h1>
                <div className="flex flex-col items-center w-full gap-2">
                    {files.map((file) => {
                        return (
                            <>
                                {file.children ? (
                                    <Folder file={file} />
                                ) : (
                                    <>
                                        <File file={file} />
                                    </>
                                )}
                            </>
                        )
                    })}
                </div>
            </main>
        </>
    )
}