import React, { useEffect, useState } from "react";
import { Crown, LogOut, UserX } from "lucide-react";
import { PlayerProfile } from "@/lib/types/profiles";
import { getCurrentProfile } from "@/lib/commands/profiles";

const ProfileSidebar: React.FC = () => {
  const [profile, setProfile] = useState<PlayerProfile>();

  useEffect(() => {
    getCurrentProfile(setProfile);
  }, []);

  return (
    <div className="w-80 bg-gray-900 border-l-2 border-gray-800 p-4 overflow-y-auto">
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
            {/* <button className="absolute bottom-0 right-0 bg-gray-800 hover:bg-gray-700 text-white p-1.5 rounded-lg transition-colors">
              <Edit2 size={14} />
            </button> */}
          </div>

          <div className="mb-2">
            <div className="flex items-center justify-center gap-2">
              <h3 className="text-white font-semibold">{profile?.data.name}</h3>
            </div>
          </div>

          {profile?.premium ? (
            <div className="text-yellow-300 flex items-center justify-center gap-1">
              <Crown size={16} />
              <span className="text-sm">Premium Account</span>
            </div>
          ) : (
            <div className="text-purple-500 flex items-center justify-center gap-1">
              <UserX size={16} />
              <span className="text-sm">Offline Account</span>
            </div>
          )}
        </div>

        <div className="pt-4 border-t border-gray-800">
          <button className="w-full bg-red-600/10 hover:bg-red-600/20 text-red-400 rounded-lg px-4 py-2 flex items-center justify-center gap-2 transition-colors">
            <LogOut size={18} />
            <span>Change Account</span>
          </button>
        </div>
      </div>
    </div>
  );
};

export default ProfileSidebar;
