"use client"

import { Instance } from "@/lib/types/instances";
import { Blocks, Gem, Joystick, Pickaxe, PlayIcon, Sword } from "lucide-react";
import { useState } from "react";

export const InstanceCard = ({ game_info, instance_type, name }: Instance) => {
  const [isHovered, setIsHovered] = useState(false);

  const getIconByTitle = (title: string) => {
    const lowerTitle = title.toLowerCase();
    if (lowerTitle.includes("survival"))
      return <Pickaxe className="w-8 h-8 text-emerald-500" />;
    if (lowerTitle.includes("pvp") || lowerTitle.includes("combat"))
      return <Sword className="w-8 h-8 text-red-500" />;
    if (lowerTitle.includes("creative"))
      return <Gem className="w-8 h-8 text-purple-500" />;
    return <Blocks className="w-8 h-8 text-blue-500" />;
  };

  return (
    <div className="bg-[#1D2026] rounded-lg overflow-hidden p-5 flex items-center">
      <div 
        className="flex gap-2 items-center"
        onMouseEnter={() => setIsHovered(true)}
        onMouseLeave={() => setIsHovered(false)}
      >
        <div 
          className="w-12 h-12 rounded-xl bg-gray-700/50 flex items-center justify-center relative"
        >
          {isHovered ? (
            <div className="bg-green-600 absolute z-20 rounded-full w-full h-full flex justify-center items-center">
              <PlayIcon className="text-white" fill="white" />
            </div>
          ) : <></>}

          <div className="z-10">
            {getIconByTitle(name)}
          </div>
        </div>

        <div className="flex flex-col gap-1">
          <h3 className="text-white font-semibold text-lg leading-tight line-clamp-1">
            {name}
          </h3>

          <div className="flex items-center gap-1">
            <Joystick className="text-gray-300" width={16} height={16} />
            <span className="text-gray-300 text-sm rounded-md line-clamp-1">
              {instance_type} {game_info.id}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
};
