import { useState } from "react";
import Sidebar from "./components/layout/Sidebar";
import HomePage from "./pages/HomePage";
import { Navbar } from "./components/layout/Navbar";
import ProfileSidebar from "./components/layout/ProfileSidebar";
import InstancesPage from "./pages/InstancesPage";
import { StorePage } from "./pages/StorePage";

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
    <div className="flex flex-col h-screen bg-gray-950 text-white overflow-hidden">
      <Navbar />

      <div className="flex h-full overflow-hidden">
        <Sidebar activeTab={activeTab} setActiveTab={setActiveTab} />

        <div className="flex w-full overflow-auto">
          <div className="w-full">{renderContent()}</div>
          <ProfileSidebar />
        </div>
      </div>
    </div>
  );
}

export default App;
