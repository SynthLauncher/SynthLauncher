import React, { useEffect, useState } from 'react';
import { PlayerProfile } from '@/lib/types/profiles';
import { getCurrentProfile } from '@/lib/commands/profiles';
import { getSkinUrl, getCapeUrl } from '@/lib/commands/skins';
import { invoke } from '@tauri-apps/api/core';
// @ts-expect-error: No types for react-minecraft-skin-viewer
import MinecraftSkinViewer from 'react-minecraft-skin-viewer';

const DEFAULT_SKIN = '/steve.png';
const DEFAULT_CAPE = null; // Use null for fallback

export const ProfileSidebar: React.FC = () => {
	const [profile, setProfile] = useState<PlayerProfile | null>(null);
	const [skinUrl, setSkinUrl] = useState<string | null>(null);
	const [capeUrl, setCapeUrl] = useState<string | null>(null);
	const [skinError, setSkinError] = useState(false);

	const isLoggedIn = !!localStorage.getItem('elyby_accessToken');

	const refreshProfile = async () => {
		const p = await getCurrentProfile();
		setProfile(p ?? null);
	};

	useEffect(() => {
		refreshProfile();

		const handler = () => refreshProfile();
		window.addEventListener('elyby-profile-updated', handler);
		return () => window.removeEventListener('elyby-profile-updated', handler);
	}, []);

	const getElyBySkinUrl = (username: string) =>
		`http://skinsystem.ely.by/skins/${username}.png?t=${Date.now()}`;

	useEffect(() => {
		setSkinError(false);
		if (profile?.data.name) {
			if (isLoggedIn) {
				setSkinUrl(getElyBySkinUrl(profile.data.name));
			} else {
				getSkinUrl(profile.data.name).then(url => setSkinUrl(url || DEFAULT_SKIN));
			}
			getCapeUrl(profile.data.name).then(url => setCapeUrl(url || DEFAULT_CAPE));
		} else {
			setSkinUrl(DEFAULT_SKIN);
			setCapeUrl(DEFAULT_CAPE);
		}
	}, [profile, isLoggedIn]);

	const handleLogout = async () => {
		localStorage.removeItem('elyby_accessToken');
		localStorage.removeItem('elyby_clientToken');
		await invoke('reset_profile_to_default');
		setProfile(null);
		window.location.reload(); // quick way to reset all state/UI
	};

	const handleRefreshSkin = async () => {
		if (profile?.data.name) {
			if (isLoggedIn) {
				setSkinUrl(getElyBySkinUrl(profile.data.name));
			} else {
				const url = await getSkinUrl(profile.data.name);
				setSkinUrl(url || DEFAULT_SKIN);
			}
			const cape = await getCapeUrl(profile.data.name);
			setCapeUrl(cape || DEFAULT_CAPE);
			// Optionally, trigger a backend refresh if needed
			// await invoke('refresh_skin_cache', { username: profile.data.name });
		}
	};

	return (
		<div className="w-80 bg-gradient-to-b from-[#41a5e7]/10 to-[#1D2026] border-l-2 border-[#2D2F32] p-4 overflow-y-auto flex flex-col items-center">
			<div className="w-24 h-24 mt-4 mb-2 flex items-center justify-center">
				<MinecraftSkinViewer
					skinUrl={skinUrl || DEFAULT_SKIN}
					width={96}
					height={96}
					onError={() => setSkinError(true)}
				/>
			</div>
			{skinError && (
				<div className="text-xs text-red-400">Failed to load skin. Showing Steve.</div>
			)}
			{capeUrl && (
				<img
					src={capeUrl}
					alt="Player Cape"
					className="w-24 h-12 object-contain mb-2 border border-[#2D2F32] bg-[#222]"
				/>
			)}
			<div className="text-center mt-2">
				<div className="text-lg font-bold text-white">{profile?.data.name}</div>
				<div className="text-xs text-gray-400">ID: {profile?.data.id}</div>
				<div className="text-xs text-gray-400">{profile?.premium ? 'Premium' : isLoggedIn ? 'Ely.by' : 'Offline'}</div>
				{isLoggedIn && (
					<>
						<button
							onClick={handleLogout}
							className="mt-4 px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded font-semibold transition"
						>
							Logout
						</button>
						<button
							onClick={handleRefreshSkin}
							className="mt-2 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded font-semibold transition"
						>
							Refresh Skin
						</button>
					</>
				)}
			</div>
		</div>
	);
};
