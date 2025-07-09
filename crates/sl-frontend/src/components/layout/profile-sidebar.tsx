import React, { useEffect, useState } from 'react';
import { PlayerProfile } from '@/lib/types/profiles';
import { getCurrentProfile } from '@/lib/commands/profiles';

export const ProfileSidebar: React.FC = () => {
	const [profile, setProfile] = useState<PlayerProfile | null>(null);

	const refreshProfile = async () => {
		const p = await getCurrentProfile();
		setProfile(p ?? null);
	};

	useEffect(() => {
		refreshProfile();
	}, []);

	return (
		<div className="w-80 bg-gradient-to-b from-[#41a5e7]/10 to-[#1D2026] border-l-2 border-[#2D2F32] p-4 overflow-y-auto flex flex-col items-center">
			<div className="w-24 h-24 mt-4 mb-2 flex items-center justify-center">

			</div>

			<div className="text-center mt-2">
				<div className="text-lg font-bold text-white">{profile?.data.name}</div>
				<div className="text-xs text-gray-400">ID: {profile?.data.id}</div>
				<div className="text-xs text-gray-400">{profile?.premium ? 'Premium' : 'Offline'}</div>
			</div>
		</div>
	);
};
