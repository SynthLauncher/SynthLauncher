import { Maximize, Minus, X } from 'lucide-react';
import { Button } from '../ui/button';
import {
	handleWindowClose,
	handleWindowMinimize,
	handleWindowMaximize,
} from '@/lib/commands/window';
import { cn } from '@/lib/utils';

const WindowControl = ({
	onClick,
	icon,
	label,
	className,
}: {
	onClick: () => void;
	icon: JSX.Element;
	label: string;
	className: string;
}) => (
	<Button
		variant="ghost"
		onClick={onClick}
		aria-label={label}
		className={cn(
			'group rounded-full p-0 flex items-center justify-center w-11 h-11 transition-all duration-300',
			className
		)}
	>
		<span className="text-white group-hover:text-black transition-all duration-300 transform group-hover:scale-105">
			{icon}
		</span>
	</Button>
);

export const Navbar = () => {
	return (
		<nav
			className="w-full flex h-[3.3rem] justify-end items-center p-1 bg-layout"
			data-tauri-drag-region
		>
			<div className="flex gap-[0.1rem]">
				<WindowControl
					onClick={handleWindowMinimize}
					icon={<Minus />}
					label="Minimize Window"
					className="hover:bg-[#8fe21a]"
				/>
				<WindowControl
					onClick={handleWindowMaximize}
					icon={<Maximize />}
					label="Maximize Window"
					className="hover:bg-[#fcbd35]"
				/>
				<WindowControl
					onClick={handleWindowClose}
					icon={<X />}
					label="Close Window"
					className="hover:bg-red-400"
				/>
			</div>
		</nav>
	);
};
