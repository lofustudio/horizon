import { useSettings } from "@/context/settings";
import { Component, Show } from "solid-js";
import SetupPage from "./setup";

const IndexPage: Component = () => {
    const { state, set } = useSettings();

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