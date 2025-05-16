import "./App.css";
import Sidebar from "./components/layout/Sidebar";
import HomePage from "./pages/HomePage";
import ProfileSidebar from "./components/layout/ProfileSidebar";
import { Navbar } from "./components/layout/Navbar";
import InstancesPage from "./pages/InstancesPage";

function App() {
  return (
    <div className="bg-[#0b0b22] h-screen flex-col overflow-hidden">
      <Navbar />

      <div className="flex overflow-hidden h-full">
        <Sidebar setActiveTab={() => {}} activeTab="home" />

        <div className="flex w-full border-l-2 border-t-2 border-gray-800 rounded-tl-2xl">
          {/* <HomePage /> */}
          <InstancesPage />
          <ProfileSidebar />
        </div>
      </div>
    </div>
  );
}

export default App;
