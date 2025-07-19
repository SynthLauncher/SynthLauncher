import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  Ellipsis,
  FolderUp,
} from "lucide-react";

export default function InstanceEllipsisMenu() {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button size="icon" variant="instance-option">
          <Ellipsis className="w-6 h-6 text-white" />
        </Button>
      </DropdownMenuTrigger>

      <DropdownMenuContent className="mt-2">
        <DropdownMenuItem>
          <FolderUp className="mr-1" /> Export the instance
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
