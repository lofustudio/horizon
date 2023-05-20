import { useEffect } from "react"
import { BrowserRouter, Route, Routes } from "react-router-dom"
import Home from "./pages"

export default function App() {
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
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<Home />} />
            </Routes>
        </BrowserRouter>
    )
}
