import React, { useEffect, useState } from 'react';
import { PlayerProfile } from '@/lib/types/profiles';
import { getCurrentProfile } from '@/lib/commands/profiles';

export const ProfileSidebar: React.FC = () => {
	const [_, setProfile] = useState<PlayerProfile>();

	useEffect(() => {
		getCurrentProfile(setProfile);
	}, []);

	return (
		<div className="w-80 bg-gradient-to-b from-[#41a5e7]/10 to-[#1D2026] border-l-2 border-[#2D2F32] p-4 overflow-y-auto"></div>
	);
};
