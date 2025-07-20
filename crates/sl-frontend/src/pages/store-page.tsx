import { useEffect, useState } from 'react';
import {
	CurseforgeStoreCard,
	ModrinthStoreCard,
} from '@/components/layout/pages/store/store-card';
import { Input } from '@/components/ui/input';
import { SearchIcon, WifiOff, Loader2, AlertCircle, ChevronFirst, ChevronLeft, ChevronRight, ChevronLast } from 'lucide-react';
import { StoreCategorySelector } from '@/components/layout/pages/store/store-category-selector';
import {
	getCurseforgeStoreSearch,
	getModrinthStoreSearch,
} from '@/lib/commands/store';
import { CurseforgeSearchResult } from '@/lib/types/store/curseforge';
import { ModrinthSearchResult } from '@/lib/types/store/modrinth';
import { Pagination, PaginationContent, PaginationItem, PaginationLink } from '@/components/ui/pagination';

export const StorePage = () => {
	const [store, setStore] = useState<'modrinth' | 'curseforge'>('modrinth');
	const [modrinthResult, setModrinthResult] = useState<ModrinthSearchResult>();
	const [curseforgeResult, setCurseforgeResult] =
		useState<CurseforgeSearchResult>();

	const [searchQuery, setSearchQuery] = useState('');
	const [category, setCategory] = useState('modpack');

	const [loading, setLoading] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const [isOnline, setIsOnline] = useState(true);

	const [page, setPage] = useState(1);

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
					const result = await getCurseforgeStoreSearch(
						searchQuery,
						classId,
						page
					);
					setCurseforgeResult(result);
				} else {
					const result = await getModrinthStoreSearch(
						searchQuery,
						category,
						page
					);
					setModrinthResult(result);
				}
			} catch (err) {
				setError('failed');
			} finally {
				setLoading(false);
			}
		};

		fetch();
	}, [searchQuery, category, isOnline, store, page]);

	const lastPage = store === 'modrinth'
		? modrinthResult?.total_hits
			? Math.ceil(modrinthResult.total_hits / 16)
			: null
		: curseforgeResult?.pagination?.totalCount
			? Math.ceil(curseforgeResult.pagination.totalCount / 16)
			: null;

	return (
		<div className="flex flex-col gap-3">
			<StoreCategorySelector
				values={['modrinth', 'curseforge']}
				displayValues={['Modrinth', 'Curseforge']}
				defaultValue="modrinth"
				onValueChange={(v) => {
					setPage(1);
					setStore(v.toLowerCase() as 'modrinth' | 'curseforge');
				}}
			/>

			<StoreCategorySelector
				// Add Data packs later
				values={['modpack', 'mod', 'shader', 'resourcepack']}
				displayValues={['Modpacks', 'Mods', 'Shaders', 'Resource Packs']}
				defaultValue="modpack"
				onValueChange={(value: string) => {
					setPage(1);
					setCategory(value.toLowerCase());
				}}
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

			{!loading &&
				!error &&
				store === 'curseforge' &&
				curseforgeResult?.data.map((hit) => <CurseforgeStoreCard hit={hit} />)}

			{!loading &&
				!error &&
				store === 'modrinth' &&
				modrinthResult?.hits.map((hit) => <ModrinthStoreCard hit={hit} />)}

			<Pagination>
				<PaginationContent>
					{page > 1 && (
						<>
							<PaginationItem>
								<PaginationLink onClick={() => setPage(1)} aria-label="Go to first page" size="icon">
									<ChevronFirst className="h-4 w-4" />
								</PaginationLink>
							</PaginationItem>
							<PaginationItem>
								<PaginationLink onClick={() => setPage(page - 1)} aria-label="Go to previous page" size="icon">
									<ChevronLeft className="h-4 w-4" />
								</PaginationLink>
							</PaginationItem>
						</>
					)}

					<PaginationItem>
						<PaginationLink onClick={() => setPage(page)} isActive>
							{page}
						</PaginationLink>
					</PaginationItem>

					{lastPage && page + 1 <= lastPage && (
						<PaginationItem>
							<PaginationLink onClick={() => setPage(page + 1)}>
								{page + 1}
							</PaginationLink>
						</PaginationItem>
					)}

					{lastPage && page + 2 <= lastPage && (
						<PaginationItem>
							<PaginationLink onClick={() => setPage(page + 2)}>
								{page + 2}
							</PaginationLink>
						</PaginationItem>
					)}

					{lastPage && page < lastPage && (
						<>
							<PaginationItem>
								<PaginationLink onClick={() => setPage(page + 1)} aria-label="Go to next page" size="icon">
									<ChevronRight className="h-4 w-4" />
								</PaginationLink>
							</PaginationItem>
							<PaginationItem>
								<PaginationLink onClick={() => setPage(lastPage)} aria-label="Go to last page" size="icon">
									<ChevronLast className="h-4 w-4" />
								</PaginationLink>
							</PaginationItem>
						</>
					)}
				</PaginationContent>
			</Pagination>


		</div>
	);
};
