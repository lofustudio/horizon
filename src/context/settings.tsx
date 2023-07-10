import { Store } from "@tauri-apps/plugin-store";
import { ParentProps, createContext, createEffect, createSignal, useContext } from "solid-js";
import { createStore } from "solid-js/store";

const SettingsContext = createContext({
    state: { setup: false, name: "" },
    get: async (key: string) => Promise.resolve({} as any),
    set: async (key: string, value: any) => Promise.resolve(false),
    remove: async (key: string) => Promise.resolve(false),
    save: async () => Promise.resolve(false),
    load: async (clear?: boolean) => Promise.resolve(false)
});

export default function SettingsContextProvider(props: ParentProps) {
    const store = new Store("settings.json");

    const [loading, setLoading] = createSignal(true);
    const [state, setState] = createStore({
        setup: false,
        name: ""
    });

    async function get(key: string) {
        const data: any = await store.get(key);
        return data;
    }

    async function set(key: string, value: any) {
        const res = await store.set(key, value).then(() => {
            setState(key as any, value);
            return true;
        }).catch(() => {
            return false;
        });

        return res;
    }

    async function remove(key: string) {
        const res = await store.delete(key).then(() => {
            setState(key as any, undefined);
            return true;
        }).catch(() => {
            return false;
        });

        return res;
    }

    async function save() {
        const res = await store.save().then(() => {
            return true;
        }).catch(() => {
            return false;
        });

        return res;
    }

    async function load(clear?: boolean) {
        const res = await store.load().then(async () => {
            setLoading(true);
            if (clear) await store.clear(); setState({ setup: false });
            const e = await store.entries();
            e.forEach((v) => {
                setState(v[0] as any, v[1]);
            });
            setLoading(false);
            return true;
        }).catch(() => {
            return false;
        });

        return res;
    }

    createEffect(async () => {
        const e = await store.entries();
        e.forEach((v) => {
            setState(v[0] as any, v[1]);
        });

        setLoading(false);
    })

    return (
        <SettingsContext.Provider value={{ state, get, set, remove, save, load }}>
            {!loading() && props.children}
        </SettingsContext.Provider>
    )
}

export const useSettings = () => useContext(SettingsContext);