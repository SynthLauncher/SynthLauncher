import { useEffect, useState } from 'react';
import { CurseforgeStoreCard, ModrinthStoreCard } from '@/components/store-card';
import { Input } from '@/components/ui/input';
import { SearchIcon, WifiOff, Loader2, AlertCircle } from 'lucide-react';
import { StoreCategorySelector } from '@/components/store-category-selector';
import { getCurseforgeStoreSearch, getModrinthStoreSearch } from '@/lib/commands/store';
import { CurseforgeSearchResult } from '@/lib/types/store/curseforge';
import { ModrinthSearchResult } from '@/lib/types/store/modrinth';

export const StorePage = () => {
	const [store, setStore] = useState<'modrinth' | 'curseforge'>('modrinth');
	const [modrinthResult, setModrinthResult] = useState<ModrinthSearchResult>();
	const [curseforgeResult, setCurseforgeResult] = useState<CurseforgeSearchResult>();

	const [searchQuery, setSearchQuery] = useState('');
	const [category, setCategory] = useState('modpack');

	const [loading, setLoading] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const [isOnline, setIsOnline] = useState(true);

	const CURSEFORGE_CLASS_IDS: Record<string, number> = {
		modpack: 4471,
		mod: 6,
		resourcepack: 12,
		shader: 6552,
	};

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
				if (store === 'curseforge') {
					const classId = CURSEFORGE_CLASS_IDS[category] ?? 4471;
					const result = await getCurseforgeStoreSearch(searchQuery, classId, 0);
					setCurseforgeResult(result);
				} else {
					const result = await getModrinthStoreSearch(searchQuery, category, 0);
					setModrinthResult(result);
				}
			} catch (err) {
				setError('failed');
			} finally {
				setLoading(false);
			}
		};

		fetch();
	}, [searchQuery, category, isOnline, store]);
	return (
		<div className="flex flex-col gap-3">
			<StoreCategorySelector
				values={['Modrinth', 'Curseforge']}
				defaultValue="modrinth"
				onValueChange={(v) => setStore(v.toLowerCase() as 'modrinth' | 'curseforge')}
			/>

			<StoreCategorySelector
				// Add Data packs later
				values={['Modpack', 'Mod', 'Shader', 'Resourcepack']}
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

			{!loading && !error && store === 'curseforge' && curseforgeResult?.data.map((hit) => (
				<CurseforgeStoreCard
					hit={hit}
				/>
			))}

			{!loading && !error && store === 'modrinth' && modrinthResult?.hits.map((hit) => (
				<ModrinthStoreCard 
					hit={hit}
				/>
			))}
		</div>
	);
};
