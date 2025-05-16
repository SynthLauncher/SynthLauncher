import React, { useEffect, useState } from "react";
import { Edit2, LogOut, UserX } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { Input } from "../ui/input";
import { getPlayerUsername } from "@/lib/commands";

const ProfileSidebar: React.FC = () => {
  const [username, setUsername] = useState("");
  const [isEditing, setIsEditing] = useState(false);

  useEffect(() => {
    getPlayerUsername(setUsername);
  }, []);

  return (
    <div className="w-80 bg-gray-900 border-l border-gray-800 p-4 overflow-y-auto">
      <div className="space-y-6">
        <div className="text-center">
          <div className="relative w-24 h-24 mx-auto mb-4">
            <div className="w-full h-full rounded-xl overflow-hidden">
              <img
                src="/steve.png"
                alt="Profile"
                className="w-full h-full object-cover"
              />
            </div>
            <button className="absolute bottom-0 right-0 bg-gray-800 hover:bg-gray-700 text-white p-1.5 rounded-lg transition-colors">
              <Edit2 size={14} />
            </button>
          </div>

          <div className="mb-2">
            {isEditing ? (
              <Input
                type="text"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                onBlur={async () => {
                  setIsEditing(false);
                  try {
                    await invoke("edit_username", { username: username });
                  } catch (error) {
                    console.log(error);
                  }
                }}
                className="bg-gray-800 text-white text-center rounded px-2 py-1 w-full"
              />
            ) : (
              <div className="flex items-center justify-center gap-2">
                <h3 className="text-white font-semibold">{username}</h3>
                <button
                  onClick={() => setIsEditing(true)}
                  className="text-gray-400 hover:text-white"
                >
                  <Edit2 size={14} />
                </button>
              </div>
            )}
          </div>

          <div className="flex items-center justify-center gap-1 text-purple-400">
            {/* <Crown size={16} /> */}
            <UserX size={16} />
            <span className="text-sm">Offline Account</span>
          </div>
        </div>

        <div className="bg-gray-800 rounded-lg p-4">
          <h4 className="text-sm font-medium text-gray-400 mb-3">
            Quick Stats
          </h4>
          <div className="space-y-2">
            <div className="flex justify-between items-center">
              <span className="text-gray-300">Playtime</span>
              <span className="text-white">127h</span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-gray-300">Instances</span>
              <span className="text-white">8</span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-gray-300">Mods Installed</span>
              <span className="text-white">45</span>
            </div>
          </div>
        </div>

        <div className="pt-4 border-t border-gray-800">
          <button className="w-full bg-red-600/10 hover:bg-red-600/20 text-red-400 rounded-lg px-4 py-2 flex items-center justify-center gap-2 transition-colors">
            <LogOut size={18} />
            <span>Sign Out</span>
          </button>
        </div>
      </div>
    </div>
  );
};

export default ProfileSidebar;
