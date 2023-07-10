import { Accessor, ParentProps, Setter, Signal, createContext, createEffect, createSignal, useContext } from "solid-js";

const initializeTheme = () => {
    let theme;
    if (typeof localStorage !== "undefined" && localStorage.getItem("theme")) {
        theme = localStorage.getItem("theme") as "light" | "dark";
    } else if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
        theme = "dark";
    } else {
        theme = "light";
    }
    return theme;
};

export const ThemeContext = createContext({
    theme: initializeTheme() as unknown as Accessor<string>,
    setTheme: void 0 as unknown as Setter<string>
});

export default function ThemeContextProvider(props: ParentProps) {
    const [theme, setTheme] = createSignal<string>(initializeTheme());

    createEffect(() => {
        const root = document.documentElement;
        if (theme() === "light") {
            root.classList.remove("dark");
            localStorage.setItem("theme", "light");
        } else {
            root.classList.add("dark");
            localStorage.setItem("theme", "dark");
        }
    });

    const value = {
        theme,
        setTheme
    }

    return (
        <ThemeContext.Provider value={value}>
            {props.children}
        </ThemeContext.Provider>
    )
}

export const useTheme = () => useContext(ThemeContext);