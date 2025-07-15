import { useEffect, useState } from 'react';
import { StoreCard } from '@/components/store-card';
import { Input } from '@/components/ui/input';
import { SearchIcon, WifiOff, Loader2, AlertCircle } from 'lucide-react';
import { StoreCategorySelector } from '@/components/store-category-selector';
import { CurseforgeSearchResponse } from '@/lib/types/store/curseforge';
import { getCurseforgeStoreSearch } from '@/lib/commands/store';

export const StorePage = () => {
	const [searchResult, setSearchResult] = useState<CurseforgeSearchResponse>();
	const [searchQuery, setSearchQuery] = useState('');
	const [category, setCategory] = useState('modpack');
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const [isOnline, setIsOnline] = useState(true);

	useEffect(() => {
		const checkOnline = () => setIsOnline(navigator.onLine);
		window.addEventListener('online', checkOnline);
		window.addEventListener('offline', checkOnline);

		checkOnline();

		return () => {
			window.removeEventListener('online', checkOnline);
			window.removeEventListener('offline', checkOnline);
		};
	}, []);

	useEffect(() => {
		const fetch = async () => {
			if (!isOnline) {
				setError('offline');
				return;
			}

			setLoading(true);
			setError(null);

			try {
				let result = await getCurseforgeStoreSearch(searchQuery, 4471, 0);
				setSearchResult(result);
			} catch (err) {
				setError('failed');
			} finally {
				setLoading(false);
			}
		};

		fetch();
	}, [searchQuery, category, isOnline]);

	return (
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

			{loading && (
				<div className="flex items-center justify-center gap-2 text-muted-foreground mt-4">
					<Loader2 className="animate-spin w-5 h-5" />
					<span>Loading...</span>
				</div>
			)}

			{error === 'offline' && (
				<div className="flex flex-col items-center justify-center h-[50vh] text-muted-foreground gap-3">
					<WifiOff className="w-16 h-16 text-gray-400" />
					<h2 className="text-xl font-semibold">You're offline</h2>
					<p className="text-sm">Please check your internet connection.</p>
				</div>
			)}

			{error === 'failed' && (
				<div className="flex items-center justify-center gap-2 text-red-500 mt-4">
					<AlertCircle className="w-5 h-5" />
					<span>Failed to load mods. Try again later.</span>
				</div>
			)}

			{!loading && !error && searchResult?.data.length === 0 && (
				<div className="flex items-center justify-center text-muted-foreground mt-4">
					<span>No results found.</span>
				</div>
			)}

			{!loading &&
				!error &&
				searchResult?.data.map((hit) => (
					<StoreCard
						author={hit.authors[0].name}
						description={hit.summary}
						downloads={hit.downloadCount}
						followers={hit.thumbsUpCount}
						imageUrl={hit.logo.url || ''}
						name={hit.name}
						slug={hit.slug}
						key={hit.slug}
					/>
				))}
		</div>
	);
};
