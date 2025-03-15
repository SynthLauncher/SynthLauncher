import { Puzzle, Trophy, Users } from "lucide-react";
import React from "react";

export default function Cards() {
  return (
    <div className="flex flex-row justify-between gap-6 px-10 pb-10 w-full">
      <Card title="Mods" icon={<Puzzle size={24} />} />
      <Card title="Achievements" icon={<Trophy size={24} />} />
      <Card title="Friends" icon={<Users size={24} />} />
    </div>
  );
}

function Card({ title, icon }: any) {
  return (
    <div className="bg-[#191919] rounded-2xl p-6 flex flex-col w-1/3 h-48 cursor-pointer transition-colors duration-200 border-2 border-neutral-700/50">
      <div className="flex flex-row items-center">
        <div className="bg-neutral-700 p-4 rounded-full text-white">
          {icon}
        </div>
        <h2 className="text-xl font-semibold ml-4 text-white">{title}</h2>
      </div>
    </div>
  );
}