import { useSettings } from "@/context/settings";
import { Component, Show } from "solid-js";
import SetupPage from "./setup";
import { useAudio } from "@/context/audio";
import { Button } from "@/components/ui/button";

const IndexPage: Component = () => {
    const { state: settings } = useSettings();
    const { state: audio, queueSong } = useAudio();

    return (
        <>
            <Show when={!settings.setup}>
                <SetupPage />
            </Show>
            <div class="py-16 px-8 flex flex-col w-full h-full">
                <h1 class="text-4xl font-bold">
                    Welcome, {settings.name}!
                </h1>
                <Button onClick={async () => { await queueSong(audio.tracks[0].id) }}>
                    Queue song
                </Button>
            </div>
        </>
    )
}

export default IndexPage;