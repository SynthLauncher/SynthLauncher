import { SidebarItemThemeProps, SidebarThemeProps } from './themes/layout';

export interface InputProps extends React.ComponentProps<'input'> {
	icon?: React.ReactNode;
}

export interface SidebarItemProps {
	icon: React.ReactNode;
	label: string;
	active?: boolean;
	onClick?: () => void;
	theme?: SidebarItemThemeProps;
}

export interface SidebarProps {
	activeTab: string;
	setActiveTab: (tab: string) => void;
	theme?: SidebarThemeProps;
}
