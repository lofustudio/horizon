import { useEffect } from "react"
import WindowBar from "./components/window/bar"
import { PlayerProvider } from "./components/player/context"
import PlayerControls from "./components/player/controls"

export default function App({ children }: { children?: React.ReactNode }) {
    useEffect(() => {
        const handleContextMenu = (e: any) => { e.preventDefault() };

        document.addEventListener("contextmenu", handleContextMenu)

        return () => {
            document.removeEventListener("contextmenu", handleContextMenu)
        }
    }, []);

    return (
        <>
            <div className="min-h-screen transition-colors border duration-250 bg-primary-100 text-primary-900 dark:bg-primary-900 dark:text-primary-100 border-black/10 dark:border-white/10">
                <PlayerProvider>
                    <div className="pb-[36px]">
                        <WindowBar />
                    </div>
                    {children}
                    <div className="pt-[64px]">
                        <PlayerControls />
                    </div>
                </PlayerProvider>
            </div>
        </>
    )
}
