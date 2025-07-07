import { CreateInstanceDialog } from '@/components/create-instance-dialog';
import { InstanceCard } from '@/components/instance-card';
import { createInstance, getInstances } from '@/lib/commands/instances';
import { Instance } from '@/lib/types/instances';
import { Plus } from 'lucide-react';
import { useEffect, useState } from 'react';

export const InstancesPage = () => {
	const [createDialogOpen, setCreateDialogOpen] = useState(false);
	const [instances, setInstances] = useState<Instance[]>([]);

	const fetchInstances = async () => {
		const all = await getInstances();
		setInstances(all);
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
		<div>
			<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-4">
				{instances.map((inst) => (
					<InstanceCard key={inst.name} {...inst} />
				))}

				<button
					onClick={() => setCreateDialogOpen(true)}
					className="bg-gray-800/50 hover:bg-sky-300/20 rounded-lg h-full p-4 border-2 border-dashed border-gray-700 hover:border-sky-600/50 transition-colors flex items-center justify-center"
				>
					<div className="flex-shrink-0 w-12 h-12 rounded-full bg-gray-700 flex items-center justify-center transition-colors">
						<Plus
							size={24}
							className="text-gray-400 group-hover:text-sky-400"
						/>
					</div>
				</button>
			</div>

			<CreateInstanceDialog
				open={createDialogOpen}
				onOpenChange={setCreateDialogOpen}
				onCreate={handleCreate}
			/>
		</div>
	);
};
