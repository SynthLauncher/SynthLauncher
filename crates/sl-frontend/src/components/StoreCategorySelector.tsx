import { StoreCategorySelectorThemeProps } from '@/lib/types/themes/store-page';
import {
	CategoryList,
	CategorySelector,
	CategoryTrigger,
} from './ui/category-selector';

export const StoreCategorySelector = ({
	values = [],
	categoryListProps,
	categoryTriggerProps,
	...props
}: StoreCategorySelectorThemeProps) => {
	return (
		<CategorySelector {...props}>
			<CategoryList defaultValue={values[0]} {...categoryListProps}>
				{values?.map((value: string) => (
					<CategoryTrigger value={value.toLowerCase()}>{value}</CategoryTrigger>
				))}
			</CategoryList>
		</CategorySelector>
	);
};
