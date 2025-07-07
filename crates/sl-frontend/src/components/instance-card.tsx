import { Instance } from '@/lib/types/instances';
import { Blocks, Gamepad, Gem, Pickaxe, Sword } from 'lucide-react';
import { useNavigate } from 'react-router-dom';

export const InstanceCard = ({ game_metadata, mod_loader, name }: Instance) => {
	const getIconByTitle = (title: string) => {
		const lowerTitle = title.toLowerCase();
		if (lowerTitle.includes('survival'))
			return <Pickaxe className="w-8 h-8 text-emerald-500" />;
		if (lowerTitle.includes('pvp') || lowerTitle.includes('combat'))
			return <Sword className="w-8 h-8 text-red-500" />;
		if (lowerTitle.includes('creative'))
			return <Gem className="w-8 h-8 text-purple-500" />;
		return <Blocks className="w-8 h-8 text-blue-500" />;
	};

	const navigate = useNavigate();

	return (
		<div 
			className="bg-[#1D2026] hover:bg-[#202627] rounded-lg overflow-hidden p-5 flex gap-2 items-center group transition-all" 
			onClick={() => {
				navigate(`/instances/${name}`, { state: name })
			}}
		>
			<div className="w-12 h-12 rounded-xl bg-[#2a2f3f] group-hover:bg-[#2e2f35] flex items-center justify-center relative transition-colors">
				<div className="z-10">{getIconByTitle(name)}</div>
			</div>

			<div className="flex flex-col gap-1">
				<h3
					className="text-white font-semibold text-lg leading-tight line-clamp-1 group-hover:text-sky-400 transition-all"
				>
					{name}
				</h3>

				<div className="flex items-center gap-1">
					<Gamepad className="text-gray-300" width={16} height={16} />
					<span className="text-gray-300 text-sm rounded-md line-clamp-1">
						{mod_loader} {game_metadata.id}
					</span>
				</div>
			</div>
		</div>
	);
};
