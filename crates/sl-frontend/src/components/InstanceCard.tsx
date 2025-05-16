import React from "react";
import {
  Play,
  MoreVertical,
  Clock,
  Blocks,
  Gem,
  Pickaxe,
  Sword,
} from "lucide-react";
import { Installation, InstanceCardProps } from "@/lib/types";
import { launchInstance, removeInstance } from "@/lib/commands";

const InstanceCard: React.FC<Installation> = ({
  name,
  info
  // modLoader,
  // modCount,
  // lastPlayed,
  // favorite,
}) => {
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
    <div className="bg-gray-800 rounded-xl overflow-hidden group transition-all duration-200 hover:shadow-lg hover:shadow-emerald-900/10 hover:-translate-y-1">
      <div className="p-4">
        <div className="flex items-center justify-between mb-4">
          <div className="w-12 h-12 rounded-xl bg-gray-700/50 flex items-center justify-center">
            {getIconByTitle(name)}
          </div>
{/* 
          {favorite && (
            <div className="bg-amber-500/90 text-amber-950 text-xs rounded-full px-2 py-0.5 font-medium">
              Favorite
            </div>
          )} */}
        </div>

        <div className="space-y-3">
          <div>
            <h3 className="text-white font-semibold text-lg leading-tight mb-1">
              {name}
            </h3>
            <div className="flex flex-wrap items-center gap-2">
              <span className="bg-gray-700/50 text-gray-300 text-xs px-2 py-1 rounded-md">
                {info.version}
              </span>
              {/* {modLoader && (
                <span className="bg-gray-700/50 text-gray-300 text-xs px-2 py-1 rounded-md">
                  {modLoader}
                </span>
              )}
              {modCount !== undefined && (
                <span className="bg-gray-700/50 text-gray-300 text-xs px-2 py-1 rounded-md">
                  {modCount} mods
                </span>
              )} */}
            </div>
          </div>

          <div className="flex items-center justify-between pt-2 border-t border-gray-700/50">
            <div className="flex items-center gap-2 text-xs text-gray-400">
              <Clock size={14} />
              {/* <span>{lastPlayed}</span> */}
            </div>

            <div className="flex items-center gap-2">
              <button
                className="text-gray-400 hover:text-white transition-colors p-1.5 hover:bg-gray-700/50 rounded-lg"
                title="More options"
                onClick={() => removeInstance(name)}
              >
                <MoreVertical size={18} />
              </button>
              <button
                className="bg-purple-600 hover:bg-purple-500 text-white rounded-lg px-3 py-1.5 flex items-center gap-1.5 transition-colors"
                title="Play instance"
                onClick={() => launchInstance(name)}
              >
                <Play size={16} />
                <span className="font-medium">Play</span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default InstanceCard;
