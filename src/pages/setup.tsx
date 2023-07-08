import { Button } from "@/components/ui/button";
import { useSettings } from "@/context/settings";
import { Component, Show, createSignal } from "solid-js";

const SetupPage: Component = () => {
    const [step, setStep] = createSignal(0);
    const [name, setName] = createSignal("");
    const { set, save } = useSettings();

    async function saveSettings() {
        await set("setup", true);
        await set("name", name());
        await save();
    }

    return (
        <>
            <div class="w-full h-full">
                <div class="flex flex-col justify-center items-center h-full gap-6">
                    <Show when={step() === 0}>
                        <div class="flex flex-col gap-2 items-center">
                            <h1 class="text-4xl font-bold">Welcome to Horizon</h1>
                            <p class="text-neutral-400">
                                Your music library, everywhere.
                            </p>
                        </div>
                        <Button onClick={() => setStep(step() + 1)} size="sm">
                            Get Started
                        </Button>
                    </Show>
                    <Show when={step() === 1}>
                        <div class="flex flex-col gap-2 items-center">
                            <h1 class="text-3xl font-bold">What should we call you?</h1>
                            <p class="text-neutral-400">
                                This will be used to identify you in the app.
                            </p>
                        </div>
                        <input class="flex rounded-md border border-black/20 dark:border-white/20 bg-white dark:bg-black py-2 px-3 text-sm placeholder:text-primary-500 focus:outline-none transition-colors focus:border-black/50 dark:focus:border-white/50 disabled:cursor-not-allowed disabled:opacity-50 w-1/3 h-9" placeholder="Name" onChange={(e) => setName(e.target.value)} value={name()} />
                        <div class="flex flex-row gap-4 items-center pt-8">
                            <Button onClick={() => setStep(step() + 1)} size="sm">
                                Confirm
                            </Button>
                        </div>
                    </Show>
                    <Show when={step() === 2}>
                        <div class="flex flex-col gap-2 items-center">
                            <h1 class="text-3xl font-bold">Is this correct?</h1>
                            <p class="text-neutral-400 text-center">
                                You can change all this later in the settings.
                            </p>
                        </div>
                        <div class="flex flex-col gap-2 items-center">
                            <p class="">Name: <span class="font-semibold">{name()}</span></p>
                        </div>
                        <div class="flex flex-row gap-4 items-center pt-8">
                            <Button onClick={() => setStep(step() - 1)} size="sm">
                                Back
                            </Button>
                            <Button onClick={async () => await saveSettings()} size="sm">
                                Confirm
                            </Button>
                        </div>
                    </Show>
                </div>
            </div >
        </>
    )
}

export default SetupPage;