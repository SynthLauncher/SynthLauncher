import { CreateInstanceDialog } from '@/components/layout/pages/instances/create-instance-dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { createInstance, getInstances } from '@/lib/commands/instances';
import { Instance } from '@/lib/types/instances';
import { Blocks, Gamepad, Gem, Pickaxe, Plus, SearchIcon, Sword } from 'lucide-react';
import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';

const InstanceCard = ({ mc_version, mod_loader, name }: Instance) => {
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
			<div className="w-12 h-12 rounded-xl bg-[#2A2F3F] group-hover:bg-[#33394D] flex shrink-0 items-center justify-center transition-colors shadow-inner">
				{getIconByTitle(name)}
			</div>

			<div className="flex flex-col justify-center gap-1 overflow-hidden">
				<h3 className="text-white text-lg font-semibold leading-tight truncate group-hover:text-sky-400 transition-colors">
					{name}
				</h3>
				<div className="flex items-center text-gray-400 text-sm gap-1 truncate">
					<Gamepad className="w-4 h-4" />
					<span className="truncate">
						{mod_loader} &middot; {mc_version}
					</span>
				</div>
			</div>
		</div>
	);
};


const CreateInstanceButton = ({
	setCreateDialogOpen,
}: {
	setCreateDialogOpen: (bool: boolean) => void;
}) => {
	return (
		<Button
			onClick={() => setCreateDialogOpen(true)}
			className="bg-gray-800/50 hover:bg-sky-300/20
			          hover:cursor-pointer rounded-lg h-full p-4 border-2
								border-dashed border-gray-700 hover:border-sky-600/50
							  transition-colors flex items-center justify-center group"
		>
			<div className="flex-shrink-0 w-12 h-12 rounded-full bg-gray-700 flex items-center justify-center transition-colors group-hover:bg-gray-600">
				<Plus size={24} className="text-gray-400 group-hover:text-gray-300" />
			</div>
		</Button>
	);
};

export const InstancesPage = () => {
	const [createDialogOpen, setCreateDialogOpen] = useState(false);
	const [instances, setInstances] = useState<Instance[]>([]);

	const fetchInstances = async () => {
		const all = await getInstances();
		setInstances(all ?? []);
	};

	useEffect(() => {
		fetchInstances();
	}, []);

	const handleCreate = async (
		name: string,
		version: string,
		modLoader: string
	) => {
		await createInstance(name, version, modLoader);
		await fetchInstances();
	};

	return (
		<div className="flex flex-col gap-4">
			<Input
				icon={<SearchIcon className="w-4 h-4" />}
				placeholder="Search instances..."
			/>

			<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-4">
				{instances.map((inst) => (
					<InstanceCard key={inst.name} {...inst} />
				))}

				<CreateInstanceButton setCreateDialogOpen={setCreateDialogOpen} />
			</div>

			<CreateInstanceDialog
				open={createDialogOpen}
				onOpenChange={setCreateDialogOpen}
				onCreate={handleCreate}
			/>
		</div>
	);
};
