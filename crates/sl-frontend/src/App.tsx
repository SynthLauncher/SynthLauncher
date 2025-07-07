import { ProfileSidebar } from './components/layout/profile-sidebar';
import { useEffect } from 'react';
import { HomePage } from './pages/home-page';
import { InstancesPage } from './pages/instances-page';
import { StorePage } from './pages/store-page';
import SettingsPage from './pages/settings-page';
import { UnknownPage } from './pages/unknown-page';
import { Navbar } from './components/layout/navbar';
import { Sidebar } from './components/layout/sidebar';
import { Route, Routes, useLocation, useNavigate } from 'react-router-dom';
import { InstancePage } from './pages/instance-page';

function AppLayout({ children }: { children: React.ReactNode }) {
	return (
		<div className="flex flex-col h-screen overflow-hidden">
			<Navbar />
			<div className="flex overflow-hidden h-full">{children}</div>
		</div>
	);
}

function MainContent() {
	const location = useLocation();
	const navigate = useNavigate();

	useEffect(() => {
		if (location.pathname === '/') navigate('/home');
	}, [location.pathname]);

	return (
		<>
			<Sidebar activeTab={location.pathname.slice(1)} />
			<div className="flex bg-neutral-900 w-full rounded-tl-2xl border-l-2 border-t-2 border-[#2D2F32]">
				<div className="w-full h-full overflow-y-auto p-6">
					<Routes>
						<Route path="/home" element={<HomePage />} />
						<Route path="/instances" element={<InstancesPage />} />
						<Route path="/instances/:name" element={<InstancePage />} />
						<Route path="/store" element={<StorePage />} />
						<Route path="/settings" element={<SettingsPage />} />
						<Route path="*" element={<UnknownPage />} />
					</Routes>
				</div>
				<ProfileSidebar />
			</div>
		</>
	);
}

function App() {
	return (
		<AppLayout>
			<MainContent />
		</AppLayout>
	);
}

export default App;
