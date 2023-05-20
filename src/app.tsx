import { useEffect } from "react"

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
        <main className="flex flex-col items-center min-h-screen p-12 pt-24">
            <h1 className='text-4xl font-black'>
                Hello world.
            </h1>
        </main>
    )
}
