import { CreateInstanceDialog } from '@/components/create-instance-dialog';
import { InstanceCard } from '@/components/instance-card';
import { createInstance, getInstances } from '@/lib/commands/instances';
import { Instance } from '@/lib/types/instances';
import { Plus } from 'lucide-react';
import { useEffect, useState } from 'react';
import { getCurrentProfile } from '@/lib/commands/profiles';

export const InstancesPage = () => {
	const [createDialogOpen, setCreateDialogOpen] = useState(false);
	const [instances, setInstances] = useState<Instance[]>([]);
	const [playerName, setPlayerName] = useState<string>('Steve');

	const fetchInstances = async () => {
		const allInstances = await getInstances();
		setInstances(allInstances);
	};

	useEffect(() => {
		fetchInstances();
		(async () => {
			const profile = await getCurrentProfile();
			if (profile?.data?.name) setPlayerName(profile.data.name);
		})();
	}, []);

	const handleCreateInstance = async (name: string, version: string, modLoader: string) => {
		await createInstance(name, version, modLoader);
		await fetchInstances();
	};

	return (
		<div className="p-6 w-full overflow-auto pb-20">
			<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-6 gap-4">
				{instances.map((instance) => (
					<InstanceCard key={instance.name} {...instance} playerName={playerName} />
				))}

				<button
					onClick={() => {
						setCreateDialogOpen(true);
					}}
					className="bg-gray-800/50 hover:bg-sky-300/20 rounded-lg h-full p-4 border-2 border-dashed border-gray-700 hover:border-sky-600/50 group transition-colors cursor-pointer"
				>
					<div className="flex flex-col items-center justify-center h-full gap-3">
						<div className="w-12 h-12 rounded-full bg-gray-700 group-hover:bg-sky-600/20 flex items-center justify-center transition-colors">
							<Plus
								size={24}
								className="text-gray-400 group-hover:text-sky-400 transition-colors"
							/>
						</div>
					</div>
				</button>
			</div>

			<CreateInstanceDialog
				onOpenChange={setCreateDialogOpen}
				open={createDialogOpen}
				onCreate={handleCreateInstance}
			/>
		</div>
	);
};
