import { useEffect, useState } from 'react';
import { StoreCard } from '@/components/StoreCard';
import { Input } from '@/components/ui/input';
import { SearchIcon } from 'lucide-react';
import { getStoreSearch } from '@/lib/commands/store';
import { StorePageThemeProps } from '@/lib/types/themes/store-page';
import { StoreCategorySelector } from '@/components/StoreCategorySelector';
import { Search } from '@/lib/types/store/modrinth';

export const StorePage = ({
	containerProps = {},
	storeCategorySelector,
	inputProps,
}: StorePageThemeProps) => {
	const [data, setData] = useState<Search>();
	const [searchQuery, setSearchQuery] = useState('');
	const [category, setCategory] = useState('modpack');

	useEffect(() => {
		const fetchData = async () => {
			const search = await getStoreSearch(searchQuery, category);
			if (search) {
				setData(search);
			}
		};

		fetchData();
	}, [searchQuery, category]);

	const { style: containerStyle, ...restContainerProps } = containerProps;

	return (
		<div
			className="bg-transparent p-6 w-full overflow-y-auto min-h-screen"
			{...restContainerProps}
			style={containerStyle}
		>
			<div className="flex flex-col gap-3">
				<StoreCategorySelector
					values={['Modrinth', 'Curseforge']}
					defaultValue="modrinth"
					{...storeCategorySelector}
				/>

				<StoreCategorySelector
					values={['Modpack', 'Mod', 'Shader', 'Resourcepack', 'Datapack']}
					defaultValue="modpack"
					onValueChange={(value: string) => setCategory(value.toLowerCase())}
				/>

				<Input
					icon={<SearchIcon className="w-4 h-4" />}
					placeholder="Search modpacks..."
					value={searchQuery}
					onChange={(e) => setSearchQuery(e.target.value)}
					{...inputProps}
				/>

				{data?.hits.map((hit) => (
					<StoreCard
						author={hit.author}
						description={hit.description}
						downloads={hit.downloads}
						followers={hit.follows}
						imageUrl={hit.icon_url ? hit.icon_url : ''}
						name={hit.title}
						slug={hit.slug}
						key={hit.slug}
					/>
				))}
			</div>
		</div>
	);
};
