import React from 'react';
import { Play, Plus, ArrowRight } from 'lucide-react';
import InstanceCard from '../components/InstanceCard';

const HomePage: React.FC = () => {
  const recentInstances = [
    {
      id: '1',
      title: 'Survival World',
      version: '1.20.2',
      modLoader: 'Forge',
      modCount: 45,
      lastPlayed: '2 hours ago',
      image: 'https://images.pexels.com/photos/1998479/pexels-photo-1998479.jpeg',
      favorite: true,
    },
    {
      id: '2',
      title: 'Creative Building',
      version: '1.19.4',
      modLoader: 'Fabric',
      modCount: 12,
      lastPlayed: 'Yesterday',
      image: 'https://images.pexels.com/photos/1072179/pexels-photo-1072179.jpeg',
      favorite: false,
    },
    {
      id: '3',
      title: 'Vanilla 1.20',
      version: '1.20.0',
      lastPlayed: '3 days ago',
      image: 'https://images.pexels.com/photos/1169754/pexels-photo-1169754.jpeg',
      favorite: false,
    },
    {
      id: '4',
      title: 'Adventure Map',
      version: '1.18.2',
      modLoader: 'Forge',
      modCount: 35,
      lastPlayed: 'Last week',
      image: 'https://images.pexels.com/photos/1287142/pexels-photo-1287142.jpeg',
      favorite: false,
    },
  ];

  return (
    <div className="p-6 overflow-y-auto w-full rounded-tl-lg">
      {/* <div className="relative overflow-hidden rounded-2xl mb-8 group">
        <img 
          src="https://images.pexels.com/photos/1998479/pexels-photo-1998479.jpeg" 
          alt="Recent game" 
          className="w-full h-72 object-cover transition-transform duration-10000 group-hover:scale-105"
        />
        
        <div className="absolute inset-0 flex flex-col justify-end p-8">
          <div className="flex items-start justify-between gap-4">
            <div>
              <div className="text-white/70 text-sm mb-1">Continue playing</div>
              <h2 className="text-white text-3xl font-bold mb-2">Survival World</h2>
              <div className="flex items-center gap-2 text-white/80 text-sm mb-4">
                <span>1.20.2</span>
                <span>•</span>
                <span>Forge</span>
                <span>•</span>
                <span>45 mods</span>
              </div>
              
              <div className="flex items-center gap-4">
                <button className="bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg px-4 py-2 flex items-center gap-2 transition-colors shadow-lg shadow-emerald-900/20">
                  <Play size={16} className="ml-0.5" />
                  <span>Play Now</span>
                </button>
                <div className="text-white/60 text-sm">Last played 2 hours ago</div>
              </div>
            </div>
          </div>
        </div>
      </div> */}
      
      {/* Recent instances section */}
      <div className="mb-8">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-white text-xl font-semibold">Recent Instances</h2>
          <button className="text-purple-400 font-semibold hover:text-purple-500 text-sm flex items-center gap-1 transition-colors">
            <span>View all</span>
            <ArrowRight size={16} />
          </button>
        </div>
        
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
          {recentInstances.map((instance) => (
            <InstanceCard
              key={instance.id}
              title={instance.title}
              version={instance.version}
              modLoader={instance.modLoader}
              modCount={instance.modCount}
              lastPlayed={instance.lastPlayed}
              image={instance.image}
              favorite={instance.favorite}
            />
          ))}
          
          <div className="bg-gray-800/50 rounded-lg h-full flex flex-col items-center justify-center p-6 border-2 border-dashed border-gray-700 hover:border-emerald-600/50 group transition-colors cursor-pointer">
            <div className="w-12 h-12 rounded-full bg-gray-700 group-hover:bg-emerald-600/20 flex items-center justify-center mb-3 transition-colors">
              <Plus size={24} className="text-gray-400 group-hover:text-emerald-400 transition-colors" />
            </div>
            <p className="text-gray-400 group-hover:text-emerald-400 text-center font-medium transition-colors">Create New Instance</p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default HomePage;