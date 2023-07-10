import { useTheme } from "@/context/theme";
import { appWindow } from "@tauri-apps/plugin-window";
import type { JSX } from "solid-js";

function WindowButton({ children, onClick }: { children: JSX.Element, onClick: () => void }) {
    return (
        <>
            <div class="inline-flex items-center justify-center text-center w-[28px] h-[28px] hover:bg-neutral-300/50 hover:dark:bg-neutral-700/50 transition-all duration-75 rounded-md" onClick={onClick}>
                {children}
            </div>
        </>
    )
}

export default function WindowBar() {
    const { theme, setTheme } = useTheme();
    return (
        <>
            <div data-tauri-drag-region class='h-[36px] z-[2000] bg-neutral-200 dark:bg-neutral-950 border-b border-black/10 dark:border-white/10 select-none relative flex justify-between top-0 left-0 right-0'>
                <div class='flex flex-col items-center justify-center px-2' draggable="false">
                    <img src="/lofu/light.png" class='w-[24px] h-[24px] block dark:hidden select-none' draggable="false" onClick={(e) => { setTheme((t: string) => (t === "light" ? "dark" : "light")); e.currentTarget.src = "/lofu/light_spinny.gif" }} />
                    <img src="/lofu/dark.png" class='w-[24px] h-[24px] hidden dark:block select-none' draggable="false" onClick={(e) => { setTheme((t: string) => (t === "light" ? "dark" : "light")); e.currentTarget.src = "/lofu/dark_spinny.gif" }} />
                </div>
                <div class='flex flex-row items-center justify-end px-1'>
                    <WindowButton onClick={() => {
                        appWindow.minimize();
                    }}>
                        <svg class='w-[18px] h-[18px]' xmlns="http://www.w3.org/2000/svg" width="18px" height="18px" viewBox="0 0 24 24">
                            <path class='block dark:hidden' fill="black" d="M19 13H5v-2h14v2Z" />
                            <path class='hidden dark:block' fill="white" d="M19 13H5v-2h14v2Z" />
                        </svg>
                    </WindowButton>
                    <WindowButton onClick={() => {
                        appWindow.toggleMaximize();
                    }}>
                        <svg class='w-[18px] h-[18px]' xmlns="http://www.w3.org/2000/svg" width="18px" height="18px" viewBox="0 0 24 24">
                            <path class='block dark:hidden' fill="black" d="M19 3H5c-1.11 0-2 .89-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2m0 2v14H5V5h14Z" />
                            <path class='hidden dark:block' fill="white" d="M19 3H5c-1.11 0-2 .89-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2m0 2v14H5V5h14Z" />
                        </svg>
                    </WindowButton>
                    <WindowButton onClick={() => {
                        appWindow.close();
                    }}>
                        <svg class='w-[20px] h-[20px]' xmlns="http://www.w3.org/2000/svg" width="20px" height="20px" viewBox="0 0 24 24">
                            <path class='block dark:hidden' fill="black" d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12L19 6.41Z" />
                            <path class='hidden dark:block' fill="white" d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12L19 6.41Z" />
                        </svg>
                    </WindowButton>
                </div>
            </div>
        </>
    )
}