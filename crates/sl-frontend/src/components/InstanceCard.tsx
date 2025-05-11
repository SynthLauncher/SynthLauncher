import React from 'react';
import { Play, MoreVertical, Clock, Calendar } from 'lucide-react';

type InstanceCardProps = {
  title: string;
  version: string;
  modLoader?: string;
  modCount?: number;
  lastPlayed: string;
  image: string;
  favorite?: boolean;
};

const InstanceCard: React.FC<InstanceCardProps> = ({
  title,
  version,
  modLoader,
  modCount,
  lastPlayed,
  image,
  favorite,
}) => {
  return (
    <div className="bg-gray-800 rounded-lg overflow-hidden group transition-all duration-200 hover:shadow-lg hover:shadow-emerald-900/10">
      <div className="relative h-35 overflow-hidden">
        <img 
          src={image} 
          alt={title} 
          className="w-full h-full object-cover transition-transform duration-500"
        />
        <div className="absolute inset-0 bg-gradient-to-t from-gray-900/80 to-transparent"></div>
        
        <div className="absolute bottom-3 left-3 right-3 flex items-end justify-between">
          <div>
            <h3 className="text-white font-bold text-lg leading-tight">{title}</h3>
            <div className="flex items-center gap-1.5 text-xs text-gray-300 mt-0.5">
              <span>{version}</span>
              {modLoader && (
                <>
                  <span className="w-1 h-1 bg-gray-500 rounded-full"></span>
                  <span>{modLoader}</span>
                </>
              )}
              {modCount !== undefined && (
                <>
                  <span className="w-1 h-1 bg-gray-500 rounded-full"></span>
                  <span>{modCount} mods</span>
                </>
              )}
            </div>
          </div>
          
          {favorite && (
            <div className="bg-amber-500/90 text-amber-950 text-xs rounded-full px-2 py-0.5 font-medium">
              Favorite
            </div>
          )}
        </div>
      </div>
      
      <div className="p-3 flex items-center justify-between">
        <div className="flex items-center gap-2 text-xs text-gray-400">
          <Clock size={14} />
          <span>{lastPlayed}</span>
        </div>
        
        <div className="flex items-center gap-2">
          <button className="text-gray-400 hover:text-white transition-colors p-1">
            <MoreVertical size={18} />
          </button>
          <button className="bg-purple-600 hover:bg-purple-500 text-white rounded-full w-8 h-8 flex items-center justify-center transition-colors">
            <Play size={16} className="ml-0.5" />
          </button>
        </div>
      </div>
    </div>
  );
};

export default InstanceCard;