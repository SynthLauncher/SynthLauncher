import { ProfileSidebar } from './components/layout/profile-sidebar';
import { useState } from 'react';
import { HomePage } from './pages/home-page';
import { InstancesPage } from './pages/instances-page';
import { StorePage } from './pages/store-page';
import { SettingsPage } from './pages/settings-page';
import { UnknownPage } from './pages/unknown-page';
import { Navbar } from './components/layout/navbar';
import { Sidebar } from './components/layout/sidebar';

function AppLayout({ children }: { children: React.ReactNode }) {
	return (
		<div className="flex flex-col h-screen bg-[#1B1D21] overflow-hidden">
			<Navbar />
			<div className="flex overflow-hidden h-full">{children}</div>
		</div>
	);
}

function MainContent({
	activeTab,
	setActiveTab,
}: {
	activeTab: string;
	setActiveTab: (tab: string) => void;
}) {
	function renderContent(activeTab: string) {
		switch (activeTab) {
			case 'home':
				return <HomePage />;
			case 'instances':
				return <InstancesPage />;
			case 'store':
				return <StorePage />;
			case 'settings':
				return <SettingsPage />;
			default:
				return <UnknownPage setActiveTab={setActiveTab} />;
		}
	}

	return (
		<>
			<Sidebar activeTab={activeTab} setActiveTab={setActiveTab} />
			<div className="flex bg-[#141518] w-full rounded-tl-2xl border-l-2 border-t-2 border-[#2D2F32]">
				<div className="w-full h-full overflow-y-auto">
					{renderContent(activeTab)}
				</div>
				<ProfileSidebar />
			</div>
		</>
	);
}

function App() {
	const [activeTab, setActiveTab] = useState('home');

	return (
		<AppLayout>
			<MainContent activeTab={activeTab} setActiveTab={setActiveTab} />
		</AppLayout>
	);
}

export default App;
