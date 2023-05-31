import { usePlayer } from "../../components/player/context";

export default function Index() {
    const { tracks, playFile } = usePlayer();
    return (
        <>
            <main className="flex flex-col items-start w-full gap-8 p-8">
                <div>
                    <h1 className="text-3xl font-bold">
                        Your Library
                    </h1>
                    <p className="text-primary dark:text-primary-400">
                        {tracks.length} tracks
                    </p>
                </div>
                <div className="flex flex-col w-full">
                    {tracks.map((t) => {
                        return (
                            <>
                                <div key={t.path} className="flex flex-row items-center justify-between p-2 transition-colors duration-150 hover:cursor-pointer hover:bg-primary-200 dark:hover:bg-primary-700/50" onClick={() => playFile(t)}>
                                    <div className="flex flex-row gap-2">
                                        <img src="https://via.placeholder.com/48x48/eee?text=Album" />
                                        <div className="flex flex-col">
                                            <h1 className="font-medium overflow-ellipsis text-md whitespace-nowrap">
                                                {t.title}
                                            </h1>
                                            <p>
                                                {t.artist}
                                            </p>
                                        </div>
                                    </div>
                                    <svg stroke="currentColor" fill="currentColor" strokeWidth="0" viewBox="0 0 3 16" height="20px" width="20px" xmlns="http://www.w3.org/2000/svg" className="h-full"><path fillRule="evenodd" d="M0 2.5a1.5 1.5 0 1 0 3 0 1.5 1.5 0 0 0-3 0zm0 5a1.5 1.5 0 1 0 3 0 1.5 1.5 0 0 0-3 0zM1.5 14a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3z"></path></svg>
                                </div>
                            </>
                        )
                    })}
                </div>

            </main>
        </>
    )
}