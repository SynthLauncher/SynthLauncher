import { Button } from "@/components/ui/button";
import { Folder } from "lucide-react";

export const InstanceFolderButton = ({ onClick }: { onClick: () => void; }) => {
    return (
        <Button size="icon" variant="instance-option" onClick={onClick}>
            <Folder className="w-6 h-6 text-white" />
        </Button>
    );
}
