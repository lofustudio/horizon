import { ParentProps, createContext, createEffect, createSignal, useContext } from "solid-js";
import { createStore } from "solid-js/store";

const AudioContext = createContext({
    state: {}
});

export default function AudioContextProvider(props: ParentProps) {
    const [loading, setLoading] = createSignal(true);
    const [state, setState] = createStore();

    createEffect(async () => { });

    return (
        <AudioContext.Provider value={{ state }}>
            {!loading() && props.children}
        </AudioContext.Provider>
    )
}

export const useSettings = () => useContext(AudioContext);