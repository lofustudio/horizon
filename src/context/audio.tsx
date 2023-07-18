// TODO: add loading state for frontend to show fallback

import { invoke } from "@tauri-apps/api";
import { Accessor, ParentProps, createContext, createEffect, createSignal, useContext } from "solid-js";
import { createStore } from "solid-js/store";

const AudioContext = createContext({
    state: {
        tracks: [{
            // TODO: add types properly
            album: "",
            artist: "",
            duration: 0,
            genre: "",
            id: 0,
            path: "",
            title: ""
        }]
    },
    queueSong: async (id: number) => Promise.resolve(false)
});

export default function AudioContextProvider(props: ParentProps) {
    const [state, setState] = createStore({ tracks: [] });

    createEffect(async () => {
        const tracks: {
            // TODO: add types properly
            album: string,
            artist: string,
            duration: number,
            genre: string,
            id: number,
            path: string,
            title: string
        }[] = await invoke("fetch_files");
        console.log(tracks);
        setState({ tracks: tracks });
    });

    async function queueSong(id: number) {
        const res = await invoke("add_file_to_queue", { file: id }).then(() => {
            return true;
        }).catch((e) => {
            return false;
        });

        return res;
    }

    return (
        <AudioContext.Provider value={{ state, queueSong }}>
            {props.children}
        </AudioContext.Provider>
    )
}

export const useAudio = () => useContext(AudioContext);