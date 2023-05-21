import { useEffect } from "react"
import WindowBar from "./components/window/bar"

export default function App({ children }: { children?: React.ReactNode }) {
    useEffect(() => {
        // define a custom handler function
        // for the contextmenu event
        const handleContextMenu = (e: any) => {
            // prevent the right-click menu from appearing
            e.preventDefault()
        }

        // attach the event listener to 
        // the document object
        document.addEventListener("contextmenu", handleContextMenu)

        // clean up the event listener when 
        // the component unmounts
        return () => {
            document.removeEventListener("contextmenu", handleContextMenu)
        }
    }, [])

    return (
        <>
            <WindowBar />
            <div className="mt-[36px]">
                {children}
            </div>
        </>
    )
}
