import { Home, Settings, Library, Store, Folder } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { openSynthLauncherFolder } from '@/lib/commands/launcher';
import {
	Tooltip,
	TooltipContent,
	TooltipTrigger,
} from '@/components/ui/tooltip';
import { useNavigate } from 'react-router-dom';
import React from 'react';

const SidebarItem = ({ icon, active, onClick }: {
	icon: React.ReactNode;
	active?: boolean;
	onClick?: () => void;
}) => {
	return (
		<Button
			className={`flex items-center gap-3 px-4 py-3 rounded-full cursor-pointer transition-colors ${active
					? 'bg-layout-accent/20 hover:bg-layout-accent/30 text-layout-accent-foreground'
					: 'bg-transparent text-layout-foreground hover:bg-layout-primary/20 hover:text-layout-primary-foreground'
				}`}
			size="icon"
			onClick={onClick}
		>
			<h1 className="text-xl">{icon}</h1>
		</Button>
	);
});

export const Sidebar = ({ activeTab }: { activeTab: string }) => {
	const navigate = useNavigate();

	interface SidebarItemConfig {
		id: string;
		label: string;
		icon: React.ReactNode;
		section: 'top' | 'bottom';
		onClick?: () => void;
	}

	const sidebarItems: SidebarItemConfig[] = [
		{
			id: 'home',
			label: 'Home',
			icon: <Home size={24} />,
			section: 'top',
		},
		{
			id: 'instances',
			label: 'Instances',
			icon: <Library size={24} />,
			section: 'top',
		},
		{
			id: 'store',
			label: 'Store',
			icon: <Store size={24} />,
			section: 'top',
		},
		{
			id: 'folder',
			label: 'Folder',
			icon: <Folder size={24} />,
			section: 'bottom',
			onClick: openSynthLauncherFolder,
		},
		{
			id: 'settings',
			label: 'Settings',
			icon: <Settings size={24} />,
			section: 'bottom',
		},
	];

	const renderItems = (section: 'top' | 'bottom') =>
		sidebarItems
			.filter((item) => item.section === section)
			.map((item) => {
				const isActive = activeTab === item.id;
				const handleClick = () => {
					if (item.onClick) {
						item.onClick();
					} else {
						navigate(`/${item.id}`);
					}
				};

				return (
					<Tooltip key={item.id}>
						<TooltipTrigger>
							<SidebarItem
								icon={item.icon}
								active={isActive}
								onClick={handleClick}
							/>
						</TooltipTrigger>
						<TooltipContent
							arrowClassName="bg-layout-secondary fill-layout-secondary"
							className="bg-layout-secondary text-layout-secondary-foreground text-md"
							side="right"
						>
							{item.label}
						</TooltipContent>
					</Tooltip>
				);
			});

	return (
		<div className="flex flex-col items-center justify-between p-2">
			<div className="flex flex-col gap-1">{renderItems('top')}</div>
			<div className="flex flex-col gap-1">{renderItems('bottom')}</div>
		</div>
	);
};
