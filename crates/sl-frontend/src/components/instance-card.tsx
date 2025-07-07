import { Instance } from '@/lib/types/instances';
import { Blocks, Gamepad, Gem, Pickaxe, Sword } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { useEffect, useState } from 'react';
import { getSkinUrl, getCapeUrl } from '@/lib/commands/skins';

const DEFAULT_SKIN = '/steve.png';
const DEFAULT_CAPE = null;

// Add playerName as a prop for skin/cape display
export const InstanceCard = ({ game_metadata, mod_loader, name, playerName }: Instance & { playerName: string }) => {
	const [skinUrl, setSkinUrl] = useState<string | null>(null);
	const [capeUrl, setCapeUrl] = useState<string | null>(null);
	const [skinLoading, setSkinLoading] = useState(true);
	const [capeLoading, setCapeLoading] = useState(true);
	const [skinError, setSkinError] = useState(false);
	const [capeError, setCapeError] = useState(false);

	useEffect(() => {
		if (playerName) {
			setSkinLoading(true);
			setCapeLoading(true);
			setSkinError(false);
			setCapeError(false);
			getSkinUrl(playerName).then(url => {
				setSkinUrl(url || DEFAULT_SKIN);
				setSkinLoading(false);
			}).catch(() => {
				setSkinUrl(DEFAULT_SKIN);
				setSkinLoading(false);
				setSkinError(true);
			});
			getCapeUrl(playerName).then(url => {
				setCapeUrl(url || DEFAULT_CAPE);
				setCapeLoading(false);
			}).catch(() => {
				setCapeUrl(DEFAULT_CAPE);
				setCapeLoading(false);
				setCapeError(true);
			});
		}
	}, [playerName]);

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
			<div className="w-12 h-12 rounded-xl bg-[#2a2f3f] group-hover:bg-[#2e2f35] flex items-center justify-center relative transition-colors flex-col">
				{skinLoading ? (
					<div className="w-10 h-10 flex items-center justify-center mb-1"><div className="animate-spin w-6 h-6 border-2 border-gray-400 border-t-transparent rounded-full"></div></div>
				) : (
					<img
						src={skinUrl || DEFAULT_SKIN}
						alt="Skin"
						className="w-10 h-10 rounded mb-1 border border-[#2D2F32] bg-[#222]"
						onError={() => { setSkinError(true); setSkinUrl(DEFAULT_SKIN); }}
					/>
				)}
				{capeLoading ? (
					<div className="w-10 h-4 flex items-center justify-center"><div className="animate-spin w-4 h-4 border-2 border-gray-400 border-t-transparent rounded-full"></div></div>
				) : (
					capeUrl && (
						<img
							src={capeUrl}
							alt="Cape"
							className="w-10 h-4 object-contain border border-[#2D2F32] bg-[#222]"
							onError={() => { setCapeError(true); setCapeUrl(DEFAULT_CAPE); }}
						/>
					)
				)}
				{!skinUrl && !skinLoading && getIconByTitle(name)}
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
