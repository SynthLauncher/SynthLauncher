import { Download, Heart } from 'lucide-react';
import { Button } from './ui/button';

export const StoreCard = ({
	name,
	author,
	description,
	downloads,
	followers,
	imageUrl,
}: {
	name: string;
	author: string;
	description: string;
	downloads: number;
	followers: number;
	imageUrl: string;
	slug: string;
}) => {
	return (
		<div className="bg-[#1D2026] rounded-lg p-5 flex gap-4 max-w-full">
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
					<div className="flex items-center gap-1">
						<Heart className="w-4 h-4" />
						<span>{followers.toLocaleString()} followers</span>
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
