import React, { useState } from 'react';
import { Plus } from 'lucide-react';
import InstanceCard from '../components/InstanceCard';
import CreateInstanceDialog from '../components/CreateInstanceDialog';
import { createInstance, getInstances, loadInstances } from '@/lib/commands';
import { Installation } from '@/lib/types';

const InstancesPage: React.FC = () => {
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [instances, setInstances] = useState([] as Installation[])

  getInstances(setInstances);

  return (
    <div className="p-6 w-full overflow-auto pb-20">
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
        {instances.map((instance) => (
          <InstanceCard key={instance.name} {...instance} />
        ))}
        
        <button 
          onClick={() => {
            setCreateDialogOpen(true) 
            loadInstances()
          }}
          className="bg-gray-800/50 rounded-lg h-full p-6 border-2 border-dashed border-gray-700 hover:border-emerald-600/50 group transition-colors cursor-pointer"
        >
          <div className="flex flex-col items-center justify-center h-full gap-3">
            <div className="w-12 h-12 rounded-full bg-gray-700 group-hover:bg-emerald-600/20 flex items-center justify-center transition-colors">
              <Plus size={24} className="text-gray-400 group-hover:text-emerald-400 transition-colors" />
            </div>
            <p className="text-gray-400 group-hover:text-emerald-400 text-center font-medium transition-colors">Create New Instance</p>
          </div>
        </button>
      </div>

      <CreateInstanceDialog onOpenChange={setCreateDialogOpen} open={createDialogOpen} onCreate={createInstance} />
    </div>
  );
};

export default InstancesPage;