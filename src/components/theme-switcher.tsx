import { useContext } from "solid-js";
import { ThemeContext } from "@/context/theme";

export default function ThemeSwitcher() {
    const { setTheme } = useContext(ThemeContext);
    return (
        <>
            <button onClick={() => setTheme((t: string) => (t === "light" ? "dark" : "light"))}>Change theme</button>
        </>
    )
}