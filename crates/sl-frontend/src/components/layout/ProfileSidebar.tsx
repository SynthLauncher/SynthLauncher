import React, { useEffect, useState } from "react";
import { PlayerProfile } from "@/lib/types/profiles";
import { getCurrentProfile } from "@/lib/commands/profiles";

const ProfileSidebar: React.FC = () => {
  const [profile, setProfile] = useState<PlayerProfile>();

  useEffect(() => {
    getCurrentProfile(setProfile);
  }, []);

  return (
    <div className="w-80 bg-gradient-to-b from-[#EC8E4F]/10 to-[#1D2026] border-l-2 border-[#2D2F32] p-4 overflow-y-auto">
      {profile?.data.name}
    </div>
  );
};

export default ProfileSidebar;

