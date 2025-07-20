import { Download } from 'lucide-react';
import { Button } from '../../../ui/button';
import { CurseforgeProject } from '@/lib/types/store/curseforge';
import { ModrinthSearchHit } from '@/lib/types/store/modrinth';

export const CurseforgeStoreCard = ({ hit }: { hit: CurseforgeProject }) => {
	return (
		<StoreCard
			name={hit.name}
			slug={hit.slug}
			author={hit.authors[0].name}
			description={hit.summary}
			downloads={hit.downloadCount}
			imageUrl={hit.logo.url || ''}
		/>
	);
};

export const ModrinthStoreCard = ({ hit }: { hit: ModrinthSearchHit }) => {
	return (
		<StoreCard
			name={hit.title}
			slug={hit.slug}
			author={hit.author}
			description={hit.description}
			downloads={hit.downloads}
			imageUrl={hit.icon_url || 'https://cdn.modrinth.com/placeholder.svg'}
		/>
	);
};

export const StoreCard = ({
	name,
	slug,
	author,
	description,
	downloads,
	imageUrl,
}: {
	name: string;
	author: string;
	description: string;
	downloads: number;
	imageUrl: string;
	slug: string;
}) => {
	return (
		<div
			className="bg-[#1D2026] rounded-lg p-5 flex gap-4 max-w-full"
			key={slug}
		>
			<img
				src={imageUrl}
				alt={`${name} icon`}
				className="w-24 h-24 rounded-md object-cover"
			/>

			<div className="flex flex-col grow justify-between">
				<h1 className="text-white text-lg font-bold">
					{name}
					<span className="text-gray-500 font-normal text-sm ml-2">
						by {author}
					</span>
				</h1>

				<p className="text-gray-400 text-sm mt-1 line-clamp-2">{description}</p>

				<div className="flex gap-4 text-gray-400 text-sm mt-2">
					<div className="flex items-center gap-1">
						<Download className="w-4 h-4" />
						<span>{downloads.toLocaleString()} downloads</span>
					</div>
				</div>
			</div>

			<div className="flex flex-col justify-between items-end ml-4">
				<Button variant="install" className="gap-2">
					<Download className="w-4 h-4" />
					Install
				</Button>
			</div>
		</div>
	);
};
