import { Home, Settings, Library, Store, Folder } from 'lucide-react';
import { Button } from '../ui/button';
import { openSynthLauncherFolder } from '@/lib/commands/launcher';
import { Tooltip, TooltipContent, TooltipTrigger } from '../ui/tooltip';
import { SidebarProps, SidebarItemProps } from '@/lib/types/components';
import { SidebarContainerProps } from '@/lib/types/themes/layout';

type SidebarItemConfig = {
	id: string;
	label: string;
	icon: React.ReactNode;
	section: 'top' | 'bottom';
	onClick?: () => void;
};

const SidebarItem = ({
	icon,
	active,
	onClick,
	theme = {},
}: SidebarItemProps) => {
	return (
		<Button
			className={`flex items-center gap-3 px-4 py-3 rounded-full cursor-pointer transition-colors ${
				active
					? 'bg-[#E78641]/20 text-[#E78641] hover:bg-[#E8A04E]/30'
					: 'bg-transparent text-gray-400 hover:bg-gray-800/50 hover:text-gray-200'
			}`}
			size="icon"
			onClick={onClick}
			{...theme?.buttonProps}
		>
			<h1 className="text-xl" {...theme?.iconProps}>
				{icon}
			</h1>
		</Button>
	);
};

const Sidebar = ({ activeTab, setActiveTab, theme = {} }: SidebarProps) => {
	const sidebarItems: SidebarItemConfig[] = [
		{ id: 'home', label: 'Home', icon: <Home size={24} />, section: 'top' },
		{
			id: 'instances',
			label: 'Instances',
			icon: <Library size={24} />,
			section: 'top',
		},
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

	const renderItems = (
		section: 'top' | 'bottom',
		sidebarContainerProps: SidebarContainerProps = {}
	) =>
		sidebarItems
			.filter((item) => item.section === section)
			.map((item) => (
				<Tooltip key={item.id} {...sidebarContainerProps?.tooltipProps}>
					<TooltipTrigger {...sidebarContainerProps?.tooltipTriggerProps}>
						<SidebarItem
							icon={item.icon}
							label={item.label}
							active={activeTab === item.id}
							onClick={
								item.onClick ? item.onClick : () => setActiveTab(item.id)
							}
							{...sidebarContainerProps?.sidebarItemProps}
						/>
					</TooltipTrigger>
					<TooltipContent
						arrowClassName="bg-[#2e3137] fill-[#2e3137]"
						className="text-md text-white bg-[#2e3137]"
						side="right"
						{...sidebarContainerProps?.tooltipContentProps}
					>
						{item.label}
					</TooltipContent>
				</Tooltip>
			));

	return (
		<div
			className="bg-[#1B1D21] h-full p-2 flex flex-col items-center justify-between"
			{...theme?.rootContainerProps}
		>
			<div className="flex flex-col gap-1" {...theme?.containerProps}>
				{renderItems('top', theme?.sidebarContainerProps)}
			</div>
			<div className="flex flex-col gap-1" {...theme?.containerProps}>
				{renderItems('bottom', theme?.sidebarContainerProps)}
			</div>
		</div>
	);
};

export default Sidebar;
