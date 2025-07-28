import { ProfileSidebar } from '@/components/layout/accounts-sidebar';
import { HomePage } from '@/pages/home-page';
import { InstancesPage } from '@/pages/instances-page';
import { StorePage } from '@/pages/store-page';
import { UnknownPage } from '@/pages/unknown-page';
import { Navbar } from '@/components/layout/navbar';
import { Sidebar } from '@/components/layout/sidebar';
import { Route, Routes } from 'react-router-dom';
import { InstancePage } from '@/pages/instance-page';
import { Toaster } from '@/components/ui/sonner';

function AppLayout({ children }: { children: React.ReactNode }) {
	return (
		<div className="font-noto-sans flex flex-col h-screen overflow-hidden overscroll-none">
			<Navbar />
			<div className="flex overflow-hidden h-full">{children}</div>
		</div>
	);
}

function MainContent() {
	return (
		<>
			<Sidebar />
			<div className="flex bg-neutral-900 w-full rounded-tl-2xl border-l-2 border-t-2 border-[#2D2F32]">
				<div className="w-full h-full overflow-y-auto p-6">
					<Routes>
						<Route path="/" element={<HomePage />} />
						<Route path="/instances" element={<InstancesPage />} />
						<Route path="/instances/:name" element={<InstancePage />} />
						<Route path="/store" element={<StorePage />} />
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
			<Toaster />
		</AppLayout>
	);
}

export default App;
