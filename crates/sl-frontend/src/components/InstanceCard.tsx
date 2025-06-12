import { Instance } from "@/lib/types/instances";
import { Blocks, Gem, Pickaxe, Sword } from "lucide-react";

export const InstanceCard = ({ game_info, instance_type, name }: Instance) => {
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
    <div className="bg-gray-800 rounded-xl overflow-hidden p-4 flex items-center">
      <div className="flex gap-2 items-center">
        <div className="w-12 h-12 rounded-xl bg-gray-700/50 flex items-center justify-center">
          {getIconByTitle(name)}
        </div>

        <div className="flex flex-col gap-1">
          <h3 className="text-white font-semibold text-lg leading-tight line-clamp-1">
            {name}
          </h3>
          <div className="flex flex-wrap items-center gap-2">
            <span className="bg-gray-700/50 text-gray-300 text-xs px-2 py-1 rounded-md">
              {instance_type} {game_info.id}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
};
