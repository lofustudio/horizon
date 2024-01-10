import React from "react"
import { appWindow } from "@tauri-apps/api/window";

function WindowButton({ children, onClick }: { children: React.ReactNode, onClick: () => void }) {
    return (
        <>
            <div className="inline-flex items-center justify-center text-center w-[36px] h-[36px] hover:bg-neutral-300/50 hover:dark:bg-neutral-700/50 transition-all duration-75" onClick={onClick}>
                {children}
            </div>
        </>
    )
}

export default function WindowBar() {
    return (
        <>
            {/* TODO: this is janky as hell, needs a re-write */}
            <div data-tauri-drag-region className='h-[36px] z-[2000] bg-background border-b border-black/10 dark:border-white/10 select-none relative flex w-full justify-between top-0 left-0 right-0'>
                <div className='flex flex-col items-center justify-center px-2 min-w-[24px]' draggable="false">
                    <img src="/lofu/light.png" className='w-[22px] h-[22px] block dark:hidden select-none' draggable="false" />
                    <img src="/lofu/dark.png" className='w-[22px] h-[22px] hidden dark:block select-none' draggable="false" />
                </div>
                <div className='flex flex-row items-center justify-end'>
                    <WindowButton onClick={() => {
                        appWindow.minimize();
                    }}>
                        <svg className='w-[20px] h-[20px]' xmlns="http://www.w3.org/2000/svg" width="18px" height="18px" viewBox="0 0 24 24">
                            <path className='block dark:hidden' fill="black" d="M19 13H5v-2h14v2Z" />
                            <path className='hidden dark:block' fill="white" d="M19 13H5v-2h14v2Z" />
                        </svg>
                    </WindowButton>
                    <WindowButton onClick={() => {
                        appWindow.toggleMaximize();
                    }}>
                        <svg className='w-[20px] h-[20px]' xmlns="http://www.w3.org/2000/svg" width="18px" height="18px" viewBox="0 0 24 24">
                            <path className='block dark:hidden' fill="black" d="M19 3H5c-1.11 0-2 .89-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2m0 2v14H5V5h14Z" />
                            <path className='hidden dark:block' fill="white" d="M19 3H5c-1.11 0-2 .89-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2m0 2v14H5V5h14Z" />
                        </svg>
                    </WindowButton>
                    <WindowButton onClick={() => {
                        appWindow.close();
                    }}>
                        <svg className='w-[22px] h-[22px]' xmlns="http://www.w3.org/2000/svg" width="20px" height="20px" viewBox="0 0 24 24">
                            <path className='block dark:hidden' fill="black" d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12L19 6.41Z" />
                            <path className='hidden dark:block' fill="white" d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12L19 6.41Z" />
                        </svg>
                    </WindowButton>
                </div>
            </div>
        </>
    )
}