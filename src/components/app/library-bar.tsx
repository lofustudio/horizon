import { Heart, Pin } from "lucide-react";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "../ui/tooltip";
import { ScrollArea } from "../ui/scroll-area";

export default function LibraryBar() {

    // generate 50 fake items
    const items = [...Array(50)].map((_, index) => ({
        name: `Item ${index + 1}`,
        image: `https://picsum.photos/seed/${index + 1}/200/200`,
        pinned: index < 5
    }))

    return (
        <>
            <ScrollArea className="h-screen">

                <div className="flex flex-col items-center w-full min-h-screen pt-[36px] border-r max-w-20">
                    <div className="flex flex-col w-full h-full p-3">
                        <TooltipProvider>
                            <Tooltip delayDuration={0}>
                                <TooltipTrigger asChild>
                                    <button className="flex flex-col items-center justify-center p-2 border rounded-md aspect-square bg-neutral-200 dark:bg-neutral-800">
                                        <Heart fill="currentColor" />
                                        <span className="sr-only">Liked Songs</span>
                                    </button>
                                </TooltipTrigger>
                                <TooltipContent side="right" className="ml-1 border dark:bg-neutral-800 border-black/10 dark:border-white/10">
                                    <p>Liked Songs</p>
                                </TooltipContent>
                            </Tooltip>
                            <ScrollArea className="h-full">
                                {items.filter((item) => item.pinned).map((item, index) => (
                                    <Tooltip delayDuration={0} key={index}>
                                        <TooltipTrigger asChild>
                                            <button className="flex flex-col items-center my-3 border rounded-md bg-neutral-800">
                                                <img src={item.image} className="w-full h-full rounded-md" />
                                                <span className="sr-only">{item.name}</span>
                                            </button>
                                        </TooltipTrigger>
                                        <TooltipContent side="right" className="flex flex-row ml-1 border dark:bg-neutral-800 border-black/10 dark:border-white/10">
                                            <span className="flex flex-row items-center gap-2">
                                                <Pin fill="currentColor" size={16} /> {" "} {item.name}
                                            </span>
                                        </TooltipContent>
                                    </Tooltip>
                                ))}
                                <hr className="w-full border-b border-black/10 dark:border-white/10" />
                                {items.filter((item) => !item.pinned).map((item, index) => (
                                    <Tooltip delayDuration={0} key={index}>
                                        <TooltipTrigger asChild>
                                            <button className="flex flex-col items-center my-3 border rounded-md bg-neutral-800">
                                                <img src={item.image} className="w-full h-full rounded-md" />
                                                <span className="sr-only">{item.name}</span>
                                            </button>
                                        </TooltipTrigger>
                                        <TooltipContent side="right" className="ml-1 border dark:bg-neutral-800 border-black/10 dark:border-white/10">
                                            <p>{item.name}</p>
                                        </TooltipContent>
                                    </Tooltip>
                                ))}
                            </ScrollArea>
                        </TooltipProvider>
                    </div>
                </div>
            </ScrollArea>
        </>
    )
}