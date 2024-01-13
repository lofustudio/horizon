import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api";

export default function YourLibraryPage() {
    return (
        <>
            <div className="flex flex-col pt-[48px] w-full">
                <div className="flex flex-col gap-4 p-4">
                    <h1 className="text-3xl font-bold">Your Library</h1>
                    <Button className="w-24" onClick={async () => invoke("play")}>
                        Test Audio
                    </Button>
                </div>
            </div>
        </>
    )
}