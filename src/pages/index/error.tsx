import { useRouteError } from "react-router"

export default function IndexError() {
    const error = useRouteError() as { statusText: string, message: string };
    console.log(error);

    return (
        <>
            <div className="flex flex-col items-center justify-center p-4">
                <h1 className="text-2xl font-bold">
                    {error.statusText || error.message === "Not Found" ? "404 - Not Found" : "Well, this is awkward..."}
                </h1>
                <p className="text-lg">{error.statusText || error.message === "Not Found" ? "This page or resource doesn't exist or you do not have access to it." : ""}</p>
            </div>
        </>
    )
}