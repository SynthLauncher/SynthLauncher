import React from "react";
import { Home, Settings, Library, Store } from "lucide-react";
import { Button } from "../ui/button";

type NavItemProps = {
  icon: React.ReactNode;
  label: string;
  active?: boolean;
  onClick: () => void;
};

const NavItem: React.FC<NavItemProps> = ({ icon, active, onClick }) => {
  return (
    <Button
      className={`flex items-center gap-3 px-4 py-3 rounded-full cursor-pointer transition-colors ${
        active
          ? "bg-[#E78641]/20 text-[#E78641] hover:bg-[#E8A04E]/30"
          : "bg-transparent text-gray-400 hover:bg-gray-800/50 hover:text-gray-200"
      }`}
      size="icon"
      onClick={onClick}
    >
      <h1 className="text-xl">{icon}</h1>
    </Button>
  );
};

type SidebarProps = {
  activeTab: string;
  setActiveTab: (tab: string) => void;
};

const Sidebar: React.FC<SidebarProps> = ({ activeTab, setActiveTab }) => {
  const navItems1 = [
    { id: "home", label: "Home", icon: <Home size={24} /> },
    { id: "instances", label: "Instances", icon: <Library size={24} /> },
    { id: "store", label: "Store", icon: <Store size={24} /> },
  ];

  const navItems2 = [
    { id: "settings", label: "Settings", icon: <Settings size={24} /> },
  ];

  return (
    <div className="bg-[#1B1D21] h-full p-2 flex flex-col items-center justify-between">
      <div className="flex flex-col gap-1">
        {navItems1.map((item) => (
          <NavItem
            key={item.id}
            icon={item.icon}
            label={item.label}
            active={activeTab === item.id}
            onClick={() => setActiveTab(item.id)}
          />
        ))}
      </div>

      <div className="flex flex-col gap-1">
        {navItems2.map((item) => (
          <NavItem
            key={item.id}
            icon={item.icon}
            label={item.label}
            active={activeTab === item.id}
            onClick={() => setActiveTab(item.id)}
          />
        ))}
      </div>
    </div>
  );
};

export default Sidebar;
