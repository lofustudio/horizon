import { Button } from "../../ui/button";
import { usePlayer } from "./context";

function DesktopPlayerControls() {
    const { paused, togglePause, currentTrack } = usePlayer();
    return (
        <>
            <div className="flex flex-row items-center justify-start w-full gap-3">
                <img src="https://via.placeholder.com/1080x1080/eee?text=Album" className="w-8 h-8" />
                <div className="flex flex-col justify-start w-full">
                    <h2 className="font-semibold text-left">
                        {currentTrack?.title === "None" ? currentTrack.path.split("Horizon\\")[1] : currentTrack?.title}
                    </h2>
                    <p className="text-sm text-left text-primary-300">
                        {currentTrack?.artist === "None" ? "Unknown artist" : currentTrack?.artist}
                    </p>
                </div>
            </div>
            <div className="flex flex-row items-center justify-center gap-6">
                <button>
                    {/* Off */}
                    <svg stroke="currentColor" fill="none" strokeWidth="2" viewBox="0 0 24 24" strokeLinecap="round" strokeLinejoin="round" height="1rem" width="1rem" xmlns="http://www.w3.org/2000/svg"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M4 12v-3c0 -1.336 .873 -2.468 2.08 -2.856m3.92 -.144h10m-3 -3l3 3l-3 3"></path><path d="M20 12v3a3 3 0 0 1 -.133 .886m-1.99 1.984a3 3 0 0 1 -.877 .13h-13m3 3l-3 -3l3 -3"></path><path d="M3 3l18 18"></path></svg>

                    {/* Loop */}
                    {/* <svg stroke="currentColor" fill="none" strokeWidth="2" viewBox="0 0 24 24" strokeLinecap="round" strokeLinejoin="round" height="1rem" width="1rem" xmlns="http://www.w3.org/2000/svg"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M4 12v-3a3 3 0 0 1 3 -3h13m-3 -3l3 3l-3 3"></path><path d="M20 12v3a3 3 0 0 1 -3 3h-13m3 3l-3 -3l3 -3"></path></svg> */}

                    {/* Loop track */}
                    {/* <svg stroke="currentColor" fill="none" strokeWidth="2" viewBox="0 0 24 24" strokeLinecap="round" strokeLinejoin="round" height="1rem" width="1rem" xmlns="http://www.w3.org/2000/svg"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M4 12v-3a3 3 0 0 1 3 -3h13m-3 -3l3 3l-3 3"></path><path d="M20 12v3a3 3 0 0 1 -3 3h-13m3 3l-3 -3l3 -3"></path><path d="M11 11l1 -1v4"></path></svg> */}
                </button>
                <button>
                    <svg stroke="currentColor" fill="currentColor" strokeWidth="0" viewBox="0 0 512 512" height="1rem" width="1rem" xmlns="http://www.w3.org/2000/svg"><path d="M170.7 256L448 448V64L170.7 256zM64 64h64v384H64z"></path></svg>
                </button>
                <Button className="flex flex-row items-center p-2" onClick={() => togglePause()}>
                    {paused ? (
                        <svg stroke="currentColor" fill="currentColor" strokeWidth="0" viewBox="0 0 512 512" height="1rem" width="1rem" xmlns="http://www.w3.org/2000/svg"><path d="M96 52v408l320-204L96 52z"></path></svg>
                    ) : (
                        <svg stroke="currentColor" fill="currentColor" strokeWidth="0" viewBox="0 0 512 512" height="1rem" width="1rem" xmlns="http://www.w3.org/2000/svg"><path d="M96 448h106.7V64H96v384zM309.3 64v384H416V64H309.3z"></path></svg>
                    )}
                </Button>
                <button>
                    <svg stroke="currentColor" fill="currentColor" strokeWidth="0" viewBox="0 0 512 512" height="1rem" width="1rem" xmlns="http://www.w3.org/2000/svg"><path d="M64 64v384l277.3-192L64 64zM384 64h64v384h-64z"></path></svg>
                </button>
                <button>
                    <svg stroke="currentColor" fill="none" strokeWidth="2" viewBox="0 0 24 24" strokeLinecap="round" strokeLinejoin="round" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M18 4l3 3l-3 3"></path><path d="M18 20l3 -3l-3 -3"></path><path d="M3 7h3a5 5 0 0 1 5 5a5 5 0 0 0 5 5h5"></path><path d="M21 7h-5a4.978 4.978 0 0 0 -3 1m-4 8a4.984 4.984 0 0 1 -3 1h-3"></path></svg>
                </button>
            </div>
            <div className="flex flex-row justify-end w-full">

            </div>
        </>
    )
}

function MobilePlayerControls() {
    const { paused, togglePause, currentTrack } = usePlayer();
    return (
        <>
            <div className="flex flex-row items-center justify-start w-full gap-3">
                <img src="https://via.placeholder.com/1080x1080/eee?text=Album" className="w-8 h-8" />
                <div className="flex flex-col justify-start w-full">
                    <h2 className="font-semibold text-left">
                        {currentTrack?.title === "None" ? currentTrack.path.split("Horizon\\")[1] : currentTrack?.title}
                    </h2>
                    <p className="text-sm text-left text-primary-300">
                        {currentTrack?.artist === "None" ? "Unknown artist" : currentTrack?.artist}
                    </p>
                </div>
            </div>
            <div className="flex flex-row items-center justify-end gap-6">
                <button>
                    <svg stroke="currentColor" fill="currentColor" strokeWidth="0" viewBox="0 0 512 512" height="1rem" width="1rem" xmlns="http://www.w3.org/2000/svg"><path d="M170.7 256L448 448V64L170.7 256zM64 64h64v384H64z"></path></svg>
                </button>
                <Button className="flex flex-row items-center p-2" onClick={() => togglePause()}>
                    {paused ? (
                        <svg stroke="currentColor" fill="currentColor" strokeWidth="0" viewBox="0 0 512 512" height="1rem" width="1rem" xmlns="http://www.w3.org/2000/svg"><path d="M96 52v408l320-204L96 52z"></path></svg>
                    ) : (
                        <svg stroke="currentColor" fill="currentColor" strokeWidth="0" viewBox="0 0 512 512" height="1rem" width="1rem" xmlns="http://www.w3.org/2000/svg"><path d="M96 448h106.7V64H96v384zM309.3 64v384H416V64H309.3z"></path></svg>
                    )}
                </Button>
                <button>
                    <svg stroke="currentColor" fill="currentColor" strokeWidth="0" viewBox="0 0 512 512" height="1rem" width="1rem" xmlns="http://www.w3.org/2000/svg"><path d="M64 64v384l277.3-192L64 64zM384 64h64v384h-64z"></path></svg>
                </button>
            </div>
        </>
    )
}

export default function PlayerControls() {
    return (
        <>
            <div className='h-[64px] z-[2000] bg-primary-200 dark:bg-primary-800 border border-black/10 dark:border-white/10 select-none flex bottom-0 left-0 right-0 fixed w-full items-center justify-between px-4'>
                {/* Desktop */}
                <div className="hidden w-full sm:flex">
                    <DesktopPlayerControls />
                </div>

                {/* Mobile */}
                <div className="flex w-full sm:hidden">
                    <MobilePlayerControls />
                </div>
            </div>
        </>
    )
}