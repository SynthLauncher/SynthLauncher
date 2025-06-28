import * as TabsPrimitive from '@radix-ui/react-tabs';

export interface StoreCategorySelectorThemeProps
	extends React.ComponentProps<typeof TabsPrimitive.Root> {
	values?: string[];
	categoryListProps?: React.ComponentProps<typeof TabsPrimitive.List>;
	categoryTriggerProps?: React.ComponentProps<typeof TabsPrimitive.Trigger>;
}

export interface StoreCardThemeProps
	extends React.DetailedHTMLProps<
		React.HTMLAttributes<HTMLDivElement>,
		HTMLDivElement
	> {
	imageProps?: React.DetailedHTMLProps<
		React.ImgHTMLAttributes<HTMLImageElement>,
		HTMLImageElement
	>;
}

export interface StorePageThemeProps {
	containerProps?: React.DetailedHTMLProps<
		React.HTMLAttributes<HTMLDivElement>,
		HTMLDivElement
	>;
	innerContainerProps?: React.DetailedHTMLProps<
		React.HTMLAttributes<HTMLDivElement>,
		HTMLDivElement
	>;
	storeCategorySelector?: StoreCategorySelectorThemeProps;
	inputProps?: React.ComponentProps<'input'>;
}
