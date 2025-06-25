import { StoreCardProps } from "@/lib/types/store";
import { Download, Heart } from "lucide-react";
import { Button } from "./ui/button";


export const StoreCard = ({
    name,
    author,
    description,
    downloads,
    followers,
    imageUrl
}: StoreCardProps) => {
    return (
        <div className="bg-[#1D2026] rounded-lg p-5 flex gap-3">
            <img src={imageUrl} alt="mod icon" className="w-24 h-24 rounded-md" />
            <div className="flex flex-col grow">
                <h1 className="text-white text-lg font-bold">{name} <span className="text-gray-500 font-normal">by {author}</span></h1>
                <p className="text-gray-500 grow">{description}</p>
            </div>
            <div className="flex flex-col items-end">
                <div className="flex gap-3 items-center text-gray-500">
                    <Download className="w-4 h-4" />
                    <p>{downloads} Downloads</p>
                </div>

                <div className="flex gap-3 items-center text-gray-500">
                    <Heart className="w-4 h-4" />
                    <p>{followers} Followers</p>
                </div>

                <Button className="mt-auto" variant="install">
                    <Download className="w-4 h-4" />
                    Install
                </Button>
            </div>
        </div>
    );
}