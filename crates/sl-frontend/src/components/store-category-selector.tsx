import {
	CategoryList,
	CategorySelector,
	CategoryTrigger,
} from './ui/category-selector';
import * as TabsPrimitive from '@radix-ui/react-tabs';

interface StoreCategorySelectorProps
	extends React.ComponentProps<typeof TabsPrimitive.Root> {
	values: string[];
}

export const StoreCategorySelector = ({
	values = [],
	...props
}: StoreCategorySelectorProps) => {
	return (
		<CategorySelector {...props}>
			<CategoryList className="bg-[#1B1D21]">
				{values?.map((value: string) => (
					<CategoryTrigger value={value.toLowerCase()}>{value}</CategoryTrigger>
				))}
			</CategoryList>
		</CategorySelector>
	);
};
