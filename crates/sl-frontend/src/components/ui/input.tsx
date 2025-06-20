import * as React from "react"

import { cn } from "@/lib/utils"
import { InputProps } from "@/lib/types/components"

function Input({ className, type, icon, ...props }: React.ComponentProps<"input"> & InputProps) {
  return (
    <div
      className={cn(
        "flex h-9 w-full items-center gap-3 rounded-md bg-[#1D2026] px-3 py-5 text-white shadow-xs transition-colors focus-within:ring-2 focus-within:ring-blue-500",
        "has-[:disabled]:cursor-not-allowed has-[:disabled]:opacity-50",
        className,
      )}
    >
      {icon && (
        <span className="text-muted-foreground flex-shrink-0">{icon}</span>
      )}

      <input
        type={type}
        className={cn(
          "placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground w-full min-w-0 flex-grow bg-transparent text-sm outline-none disabled:pointer-events-none",
        )}
        {...props}
      />
    </div>
  )
}

export { Input }
