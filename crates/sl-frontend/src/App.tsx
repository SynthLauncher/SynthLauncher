import { useState } from "react";
import Sidebar from "./components/layout/Sidebar";
import { Navbar } from "./components/layout/Navbar";
import ProfileSidebar from "./components/layout/ProfileSidebar";
import { HomePage } from "./pages/HomePage";
import { InstancesPage } from "./pages/InstancesPage";
import { StorePage } from "./pages/StorePage";
import { SettingsPage } from "./pages/SettingsPage";

function App() {
  const [activeTab, setActiveTab] = useState("home");

  const renderContent = () => {
    switch (activeTab) {
      case "home":
        return <HomePage />;
      case "instances":
        return <InstancesPage />;
      case "store":
        return <StorePage />;
      case "settings":
        return <SettingsPage />;
      default:
        return (
          <div className="flex items-center justify-center h-full p-8">
            <div className="text-center">
              <h2 className="text-white text-2xl font-bold mb-4">
                Unknown Page
              </h2>
              <p className="text-gray-400">Please return to the home page!</p>
            </div>
          </div>
        );
    }
  };

  return (
    <div className="flex flex-col h-screen bg-[#1B1D21] overflow-hidden">
      <Navbar />

      <div className="flex h-full overflow-hidden">
        <Sidebar activeTab={activeTab} setActiveTab={setActiveTab} />

        <div className="flex w-full bg-[#141518] rounded-tl-2xl border-t-2 border-l-2 border-[#2D2F32]">
          <div className="w-full">{renderContent()}</div>
          <ProfileSidebar />
        </div>
      </div>
    </div>
  );
}

export default App;
