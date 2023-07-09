import { useEffect, useState } from "react"
import WindowBar from "./components/window/bar"
import { PlayerProvider } from "./components/player/context"
import PlayerControls from "./components/player/controls"
import { platform } from "@tauri-apps/plugin-os"

export default function App({ children }: { children?: React.ReactNode }) {
    const [mobile, setMobile] = useState(false);
    useEffect(() => {
        platform().then((platform) => {
            if (platform === "android" || platform === "ios") {
                setMobile(true);
            }
        });

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
                    <div className="pb-[36px] data-[mobile=true]:hidden" data-mobile={mobile}>
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
