import { useEffect, useState } from 'react';
import { StoreCard } from '@/components/store-card';
import { Input } from '@/components/ui/input';
import { SearchIcon } from 'lucide-react';
import { getStoreSearch } from '@/lib/commands/store';
import { StoreCategorySelector } from '@/components/store-category-selector';
import { Search } from '@/lib/types/store/modrinth';

export const StorePage = () => {
	const [search, setSearch] = useState<Search>();
	const [searchQuery, setSearchQuery] = useState('');
	const [category, setCategory] = useState('modpack');

	useEffect(() => {
		const fetch = async () => {
			await getStoreSearch(searchQuery, category, setSearch);
		};

		fetch();
	}, [searchQuery, category]);

	return (
		<div className="bg-transparent p-6 w-full overflow-y-auto min-h-screen">
			<div className="flex flex-col gap-3">
				<StoreCategorySelector
					values={['Modrinth', 'Curseforge']}
					defaultValue="modrinth"
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
				/>

				{search?.hits.map((hit) => (
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
