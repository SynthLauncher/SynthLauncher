import { Button } from "@/components/ui/button";
import { Play } from "lucide-react";

export const InstancePlayButton = ({ onClick, isRunning }: { onClick: () => void, isRunning: boolean }) => {
    return (
        <Button
            className={
                `bg-green-500 hover:bg-green-600 text-white 
                font-semibold rounded-md flex items-center gap-2 
                shadow transition disabled:opacity-50 disabled:cursor-not-allowed`
            }
            size="instance-play"
            onClick={onClick}
            disabled={isRunning}
        >
            <Play className="w-6 h-6" />
            <span>{isRunning ? 'Running...' : 'Play'}</span>
        </Button>

    );
}
