import { useSettings } from "@/context/settings";
import { Component, Show, createEffect } from "solid-js";
import SetupPage from "./setup";
import { Howl } from "howler";

const IndexPage: Component = () => {
    const { state } = useSettings();

    createEffect(() => {
        var sound = new Howl({
            src: ['https://listen.moe/fallback'],
            html5: true,
            volume: 0.1
        });

        sound.play();
    });

    return (
        <>
            <Show when={!state.setup}>
                <SetupPage />
            </Show>
            <div class="py-16 px-8 flex flex-col w-full h-full">
                <h1 class="text-4xl font-bold">
                    Welcome, {state.name}!
                </h1>
            </div>
        </>
    )
}

export default IndexPage;