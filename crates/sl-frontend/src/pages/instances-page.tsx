import { CreateInstanceButton } from '@/components/layout/pages/instances/create-instance-button';
import { CreateInstanceDialog } from '@/components/layout/pages/instances/create-instance-dialog';
import { InstanceCard } from '@/components/layout/pages/instances/instance-card';
import { createInstance, getInstances } from '@/lib/commands/instances';
import { Instance } from '@/lib/types/instances';
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
