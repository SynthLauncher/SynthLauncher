import { Home, Settings, Library, Store, Folder } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { openSynthLauncherFolder } from '@/lib/commands/launcher';
import {
	Tooltip,
	TooltipContent,
	TooltipTrigger,
} from '@/components/ui/tooltip';
import { useLocation, useNavigate } from 'react-router-dom';

const SidebarItem = ({ id, label, icon, active, onClick }: {
	id: string,
	label: string,
	icon: React.ReactNode;
	active?: boolean;
	onClick?: () => void;
}) => {
	return (
		<Tooltip key={id}>
			<TooltipTrigger>
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
			</TooltipTrigger>
			<TooltipContent
				arrowClassName="bg-layout-secondary fill-layout-secondary"
				className="bg-layout-secondary text-layout-secondary-foreground text-md"
				side="right"
			>
				{label}
			</TooltipContent>
		</Tooltip>

	);
};

export const Sidebar = () => {
	const navigate = useNavigate();
	const location = useLocation();

	return (
		<div className="flex flex-col items-center justify-between p-2">
			<div className="flex flex-col gap-1">
				<SidebarItem 
					icon={<Home size={24} />} label='Home' 
					id='home' onClick={() => navigate('/')} 
					active={location.pathname === '/'} 
				/>
				<SidebarItem 
					icon={<Library size={24} />} label='Instances' 
					id='instance' onClick={() => navigate('/instances')} 
					active={location.pathname === '/instances'} 
				/>
				<SidebarItem 
					icon={<Store size={24} />} label='Store' 
					id='store' onClick={() => navigate('/store')} 
					active={location.pathname === '/store'} 
				/>
			</div>
			<div className="flex flex-col gap-1">
				<SidebarItem icon={<Folder size={24} />} label='Folder' id='folder' onClick={openSynthLauncherFolder} />
				<SidebarItem icon={<Settings size={24} />} label='Settings' id='settings' />
			</div>

		</div>
	);
};
