import {
	CategoryList,
	CategorySelector,
	CategoryTrigger,
} from '../../../ui/category-selector';
import * as TabsPrimitive from '@radix-ui/react-tabs';

interface StoreCategorySelectorProps
	extends React.ComponentProps<typeof TabsPrimitive.Root> {
	values: string[];
	displayValues: string[];
}

export const StoreCategorySelector = ({
	values = [],
	displayValues = [],
	...props
}: StoreCategorySelectorProps) => {
	return (
		<CategorySelector {...props}>
			<CategoryList className="bg-[#262729]">
				{values?.map((value, index) => (
					<CategoryTrigger className="hover:cursor-pointer" value={value}>
						{displayValues[index]}
					</CategoryTrigger>
				))}
			</CategoryList>
		</CategorySelector>
	);
};
