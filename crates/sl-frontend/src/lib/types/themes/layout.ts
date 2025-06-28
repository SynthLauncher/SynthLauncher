import { ButtonProps } from '@/components/ui/button';
import { LucideProps } from 'lucide-react';
import * as TooltipPrimitive from '@radix-ui/react-tooltip';
import { SidebarItemProps } from '../components';
import { TooltipContentProps } from '@/components/ui/tooltip';

export interface LayoutThemeProps {
	rootContainerProps?: React.DetailedHTMLProps<
		React.HTMLAttributes<HTMLDivElement>,
		HTMLDivElement
	>;
	containerProps?: React.DetailedHTMLProps<
		React.HTMLAttributes<HTMLDivElement>,
		HTMLDivElement
	>;
	mainWindowContainerProps?: React.DetailedHTMLProps<
		React.HTMLAttributes<HTMLDivElement>,
		HTMLDivElement
	>;
	contentContainerProps?: React.DetailedHTMLProps<
		React.HTMLAttributes<HTMLDivElement>,
		HTMLDivElement
	>;
	navbarProps?: NavbarThemeProps;
	sidebarThemeProps?: SidebarThemeProps;
	profileSidebarThemeProps?: ProfileSidebarThemeProps;
}

export interface NavbarThemeProps {
	rootContainerProps?: React.DetailedHTMLProps<
		React.HTMLAttributes<HTMLElement>,
		HTMLElement
	>;
	containerProps?: React.DetailedHTMLProps<
		React.HTMLAttributes<HTMLDivElement>,
		HTMLDivElement
	>;
	minimizeButtonProps?: ButtonProps;
	maximizeButtonProps?: ButtonProps;
	closeButtonProps?: ButtonProps;
	minimizeIconProps?: React.ForwardRefExoticComponent<
		Omit<LucideProps, 'ref'> & React.RefAttributes<SVGSVGElement>
	>;
	maximizeIconProps?: React.ForwardRefExoticComponent<
		Omit<LucideProps, 'ref'> & React.RefAttributes<SVGSVGElement>
	>;
	closeIconProps?: React.ForwardRefExoticComponent<
		Omit<LucideProps, 'ref'> & React.RefAttributes<SVGSVGElement>
	>;
}

export interface SidebarItemThemeProps {
	buttonProps?: ButtonProps;
	iconProps?: React.DetailedHTMLProps<
		React.HTMLAttributes<HTMLHeadingElement>,
		HTMLHeadingElement
	>;
}

export interface SidebarContainerProps {
	tooltipProps?: React.ComponentProps<typeof TooltipPrimitive.Root>;
	tooltipTriggerProps?: React.ComponentProps<typeof TooltipPrimitive.Trigger>;
	tooltipContentProps?: TooltipContentProps;
	sidebarItemProps?: SidebarItemProps;
}

export interface SidebarThemeProps {
	rootContainerProps?: React.DetailedHTMLProps<
		React.HTMLAttributes<HTMLDivElement>,
		HTMLDivElement
	>;
	containerProps?: React.DetailedHTMLProps<
		React.HTMLAttributes<HTMLDivElement>,
		HTMLDivElement
	>;
	sidebarContainerProps?: SidebarContainerProps;
}

export interface ProfileSidebarThemeProps {}
