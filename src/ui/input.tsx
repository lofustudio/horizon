import * as React from "react"

import { cn } from "../lib/utils"

export type InputProps = React.InputHTMLAttributes<HTMLInputElement>

const Input = React.forwardRef<HTMLInputElement, InputProps>(
    ({ className, ...props }, ref) => {
        return (
            <input
                className={cn(
                    "flex h-10 w-full rounded-md border border-black/10 dark:border-white/10 bg-white dark:bg-black py-2 px-3 text-sm placeholder:text-primary-500 focus:outline-none transition-colors focus:border-black/50 dark:focus:border-white/50 disabled:cursor-not-allowed disabled:opacity-50",
                    className
                )}
                ref={ref}
                {...props}
            />
        )
    }
)
Input.displayName = "Input"

export { Input }