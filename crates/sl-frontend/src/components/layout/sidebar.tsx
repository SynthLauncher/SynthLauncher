import { Home, Settings, Library, Store, Folder } from 'lucide-react';
import { Button } from '../ui/button';
import { openSynthLauncherFolder } from '@/lib/commands/launcher';
import { Tooltip, TooltipContent, TooltipTrigger } from '../ui/tooltip';
import { useNavigate } from 'react-router-dom';

const SidebarItem = ({
	icon,
	active,
	onClick,
}: {
	icon: React.ReactNode;
	active?: boolean;
	onClick?: () => void;
}) => {
	return (
		<Button
			className={`flex items-center gap-3 px-4 py-3 rounded-full cursor-pointer transition-colors ${active
					? 'bg-[#41a5e7]/20 text-[#41a5e7] hover:bg-[#41a5e7]/30'
					: 'bg-transparent text-gray-400 hover:bg-gray-800/50 hover:text-gray-200'
				}`}
			size="icon"
			onClick={onClick}
		>
			<h1 className="text-xl">{icon}</h1>
		</Button>
	);
};

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
		{ id: 'home', label: 'Home', icon: <Home size={24} />, section: 'top' },
		{ id: 'instances', label: 'Instances', icon: <Library size={24} />, section: 'top' },
		{ id: 'store', label: 'Store', icon: <Store size={24} />, section: 'top' },
		{
			id: '_folder',
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
							<SidebarItem icon={item.icon} active={isActive} onClick={handleClick} />
						</TooltipTrigger>
						<TooltipContent
							arrowClassName="bg-[#2e3137] fill-[#2e3137]"
							className="text-md text-white bg-[#2e3137]"
							side="right"
						>
							{item.label}
						</TooltipContent>
					</Tooltip>
				);
			});

	return (
		<div className="bg-[#1B1D21] h-full p-2 flex flex-col items-center justify-between">
			<div className="flex flex-col gap-1">{renderItems('top')}</div>
			<div className="flex flex-col gap-1">{renderItems('bottom')}</div>
		</div>
	);
};
