import { Plus } from 'lucide-react';
import { Button } from './ui/button';

export const CreateInstanceButton = ({
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
