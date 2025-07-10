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
			<CategoryList className="bg-[#262729]">
				{values?.map((value: string) => (
					<CategoryTrigger className='hover:cursor-pointer' value={value.toLowerCase()}>{value}</CategoryTrigger>
				))}
			</CategoryList>
		</CategorySelector>
	);
};
