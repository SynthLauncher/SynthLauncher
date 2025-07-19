import { Button } from "@/components/ui/button";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover";
import { Trash } from "lucide-react";

export const InstanceDeleteFilePopover = () => {
    return (
        <Popover>
            <PopoverTrigger asChild>
                <Button className={
                    `w-12 h-12 flex items-center justify-center 
            bg-neutral-700 hover:bg-neutral-600 
            rounded-md shadow transition`} size="icon">
                    <Trash className='w-6 h-6 text-red-400' />
                </Button>
            </PopoverTrigger>

            <PopoverContent className='w-80'>
                <div className='flex flex-col items-center gap-4'>
                    <div className='flex aspect-square size-12 items-center justify-center rounded-full bg-red-500/10'>
                        <Trash className='text-destructive size-6' />
                    </div>
                    <div className='space-y-2 text-center'>
                        <div className='font-semibold text-balance'>Are you sure you want to delete this file?</div>
                    </div>

                    <div className='grid w-full grid-cols-2 gap-2'>
                        <Button variant='secondary' size='sm'>
                            Cancel
                        </Button>
                        <Button variant='destructive' size='sm'>
                            Delete File
                        </Button>
                    </div>
                </div>
            </PopoverContent>
        </Popover>
    );
}