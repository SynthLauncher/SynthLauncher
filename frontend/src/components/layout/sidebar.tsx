import Image from "next/image";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "../ui/dialog";

export const Sidebar = () => {
  return (
    <div className="bg-[#2e2d2d] h-screen w-[75px] flex flex-col items-center p-1">
      <Dialog>
        <DialogTrigger>
          <Image
            className="rounded-full object-cover mt-2"
            src="/icon1.png"
            alt="icon"
            width={50}
            height={50}
          />
        </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Are you absolutely sure?</DialogTitle>
            <DialogDescription>
              This action cannot be undone. This will permanently delete your
              account and remove your data from our servers.
            </DialogDescription>
          </DialogHeader>
        </DialogContent>
      </Dialog>
    </div>
  );
};
