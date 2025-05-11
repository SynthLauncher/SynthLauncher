import React from "react";

const ProfileSidebar = () => {
  return (
    <div className="bg-gray-900 w-96 h-full p-4 flex flex-col">
      <div className="flex flex-col justify-center items-center gap-1">
        <img src="/steve.png" alt="Profile" width={105} height={105} />

        <h1 className="text-white text-2xl font-semibold">STierProgrammer</h1>
      </div>
    </div>
  );
};

export default ProfileSidebar;
