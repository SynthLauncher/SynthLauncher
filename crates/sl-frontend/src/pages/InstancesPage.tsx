import { CreateInstanceDialog } from "@/components/CreateInstanceDialog";
import { InstanceCard } from "@/components/InstanceCard";
import { createInstance, getInstances } from "@/lib/commands/instances";
import { Instance } from "@/lib/types/instances";
import { Plus } from "lucide-react";
import React, { useEffect, useState } from "react";

export const InstancesPage: React.FC = () => {
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [instances, setInstances] = useState<Instance[]>([]);

  useEffect(() => {
    const fetchData = async () => {
      await getInstances(setInstances);
    };

    fetchData();
  }, []);

  return (
    <div className="p-6 w-full overflow-auto pb-20">
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-6 gap-4">
        {instances.map((instance) => (
          <InstanceCard key={instance.name} {...instance} />
        ))}
        
        <button 
          onClick={() => {
            setCreateDialogOpen(true);
          }}
          className="bg-gray-800/50 hover:bg-orange-300/20 rounded-lg h-full p-4 border-2 border-dashed border-gray-700 hover:border-orange-600/50 group transition-colors cursor-pointer"
        >
          <div className="flex flex-col items-center justify-center h-full gap-3">
            <div className="w-12 h-12 rounded-full bg-gray-700 group-hover:bg-orange-600/20 flex items-center justify-center transition-colors">
              <Plus size={24} className="text-gray-400 group-hover:text-orange-400 transition-colors" />
            </div>
          </div>
        </button>
      </div>

      <CreateInstanceDialog onOpenChange={setCreateDialogOpen} open={createDialogOpen} onCreate={createInstance} /> 
    </div>
  );
};
