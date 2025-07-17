import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  Copy,
  Ellipsis,
  ExternalLink,
  Eye,
  MessageCircle,
  Search,
  User,
} from "lucide-react";

export default function DropdownMenuWithSubMenu() {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button className={
            `w-10 h-10 flex items-center justify-center 
            bg-neutral-700 hover:bg-neutral-600 
            rounded-md shadow transition`}>
          <Ellipsis className="w-6 h-6 text-white" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="mt-2">
        <DropdownMenuItem>
          <Eye className="mr-1" /> Open conversation details
        </DropdownMenuItem>
        <DropdownMenuItem>
          <User className="mr-1" /> View full profile
        </DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem>
          <MessageCircle className="mr-1" /> Start conversation
        </DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuSub>
          <DropdownMenuSubTrigger>
            <Copy className="mr-1" />
            Copy
          </DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem>Copy name</DropdownMenuItem>
            <DropdownMenuItem>Copy email</DropdownMenuItem>
            <DropdownMenuItem>Copy link</DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>
        <DropdownMenuItem>
          <Search className="mr-1" /> Search in conversation
        </DropdownMenuItem>
        <DropdownMenuItem>
          <ExternalLink className="mr-1" /> Open in new window
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
