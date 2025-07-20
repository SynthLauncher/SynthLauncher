import { Instance } from '@/lib/types/instances';
import { Blocks, Gamepad, Gem, Pickaxe, Sword } from 'lucide-react';
import { useNavigate } from 'react-router-dom';

export const InstanceCard = ({ game_metadata, mod_loader, name }: Instance) => {
	const getIconByTitle = (title: string) => {
		const lowerTitle = title.toLowerCase();
		if (lowerTitle.includes('survival'))
			return <Pickaxe className="w-8 h-8 text-emerald-400" />;
		if (lowerTitle.includes('pvp') || lowerTitle.includes('combat'))
			return <Sword className="w-8 h-8 text-rose-400" />;
		if (lowerTitle.includes('creative'))
			return <Gem className="w-8 h-8 text-violet-400" />;
		return <Blocks className="w-8 h-8 text-blue-400" />;
	};

	const navigate = useNavigate();

	return (
		<div
			className="rounded-xl bg-[#1D1F2B] p-5 flex gap-4 items-center group cursor-pointer transition-colors hover:shadow-lg hover:bg-[#232631]/90"
			onClick={() => {
				navigate(`/instances/${name}`, { state: name });
			}}
		>
			<div className="w-12 h-12 rounded-xl bg-[#2A2F3F] group-hover:bg-[#33394D] flex items-center justify-center transition-colors shadow-inner">
				{getIconByTitle(name)}
			</div>

			<div className="flex flex-col justify-center gap-1 overflow-hidden">
				<h3 className="text-white text-lg font-semibold leading-tight truncate group-hover:text-sky-400 transition-colors">
					{name}
				</h3>
				<div className="flex items-center text-gray-400 text-sm gap-1 truncate">
					<Gamepad className="w-4 h-4" />
					<span className="truncate">
						{mod_loader} &middot; {game_metadata.id}
					</span>
				</div>
			</div>
		</div>
	);
};
