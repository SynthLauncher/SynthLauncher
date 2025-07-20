import {
	getGameInfo,
	getInstances,
	killInstance,
	launchInstance,
} from '@/lib/commands/instances';
import { openInstanceFolder } from '@/lib/commands/launcher';
import { GameInfo, Instance } from '@/lib/types/instances';
import { Blocks, Ellipsis, FolderUp, Loader2 } from 'lucide-react';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { InstanceFolderButton } from '@/components/layout/pages/instance/instance-folder-button';
import { InstancePlayButton } from '@/components/layout/pages/instance/instance-play-button';
import { ToastInfo, ToastSuccess } from '@/components/toasters';
import { useTranslation } from 'react-i18next';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu';
import { Button } from '@/components/ui/button';

export const InstancePage = () => {
	const { t: tr } = useTranslation('instance-page');
	const tabs = ['content', 'logs', 'saves', 'screenshots', 'console'] as const;

	const { name } = useParams<{ name: string }>();
	const [instance, setInstance] = useState<Instance>();
	const [tab, setTab] = useState<
		'content' | 'logs' | 'saves' | 'screenshots' | 'console'
	>('content');
	const [isRunning, setIsRunning] = useState(false);
	const [isLoading, setIsLoading] = useState(false);
	const [gameInfo, setGameInfo] = useState<GameInfo>();

	useEffect(() => {
		const fetchAll = async () => {
			setIsLoading(true);
			let instances = await getInstances();
			const match = instances?.find((inst) => inst.name === name);
			setInstance(match);
			setIsLoading(false);
		};
		fetchAll();
	}, []);

	useEffect(() => {
		if (!instance) return;
		const fetchAll = async () => {
			const gameInfo = await getGameInfo(instance.name);
			setGameInfo(gameInfo);
		};
		fetchAll();
	}, [instance]);

	if (isLoading) {
		return (
			<div className="flex items-center justify-center gap-2 text-muted-foreground mt-4">
				<Loader2 className="animate-spin w-5 h-5" />
				<span>Loading...</span>
			</div>
		);
	}

	if (!instance) {
		return (
			<div className="p-8 text-center text-lg text-red-400 font-medium">
				Instance not found.
			</div>
		);
	}

	return (
		<div className="w-full px-4 md:px-8 py-6 min-h-screen text-white">
			<div className="flex flex-col lg:flex-row items-start lg:items-center justify-between bg-[#2d303f] rounded-2xl p-6 gap-6 shadow-lg">
				<div className="flex gap-6 items-center w-full lg:w-auto">
					<div className="w-20 h-20 sm:w-24 sm:h-24 bg-[#424555] rounded-xl flex justify-center items-center shrink-0">
						<Blocks className="w-10 h-10 sm:w-14 sm:h-14 text-blue-400" />
					</div>

					<div className="flex flex-col gap-2">
						<h1 className="text-2xl sm:text-3xl font-bold break-all">
							{instance.name}
						</h1>
						<div className="inline-block bg-blue-500/20 text-blue-400 text-sm font-semibold px-3 py-1 rounded-full w-fit">
							{instance.mod_loader}
						</div>
					</div>
				</div>

				<div className="flex flex-wrap gap-2 w-full lg:w-auto justify-start lg:justify-end">
					<InstancePlayButton
						onClick={async () => {
							setIsRunning(true);
							try {
								ToastInfo('Instance has begun launching...');

								await launchInstance(instance.name);

								ToastSuccess('Instance has been closed successfully.');
							} finally {
								setIsRunning(false);
							}
						}}
						isRunning={isRunning}
					/>

					<DropdownMenu>
						<DropdownMenuTrigger asChild>
							<Button size="icon" variant="instance-option">
								<Ellipsis className="w-6 h-6 text-white" />
							</Button>
						</DropdownMenuTrigger>

						<DropdownMenuContent className="mt-2">
							<DropdownMenuItem onClick={async () => killInstance(instance.name)}>
								<FolderUp className="mr-1" /> Kill Instance
							</DropdownMenuItem>
						</DropdownMenuContent>
					</DropdownMenu>

					<InstanceFolderButton
						onClick={async () => openInstanceFolder(instance.name)}
					/>
				</div>
			</div>

			<div className="mt-6 border-b border-neutral-700 flex gap-2 sm:gap-4 flex-wrap">
				{tabs.map((t) => (
					<button
						key={t}
						onClick={() => setTab(t)}
						className={`capitalize px-3 py-2 font-medium border-b-2 text-sm sm:text-base ${tab === t
								? 'text-blue-400 border-blue-400'
								: 'text-neutral-400 border-transparent hover:text-white'
							} transition`}
					>
						{tr(`tabs.${t}`)}
					</button>
				))}
			</div>

			<div className="mt-6 p-4 sm:p-6 bg-neutral-800 rounded-xl text-neutral-300 text-base min-h-[200px] flex items-center justify-center">
				{tab === 'content' && (
					<div className="text-center w-full">
						<p className="text-white mb-4">
							You haven’t added any content to this instance yet.
						</p>
						<button className="bg-neutral-700 hover:bg-neutral-600 px-4 py-2 rounded-md text-white font-medium transition">
							+ Install content
						</button>
					</div>
				)}

				{tab === 'logs' && (
					<div className="text-neutral-500">Logs tab coming soon...</div>
				)}

				{tab === 'saves' && (
					<div className="w-full flex flex-col gap-4">
						{gameInfo?.worlds.map((world, index) => (
							<div
								key={index}
								className="bg-neutral-700 rounded-xl shadow-lg hover:shadow-xl transition flex items-center p-4 gap-4"
							>
								<img
									src={`data:image/png;base64,${world.icon}`}
									alt={world.name}
									className="w-14 h-14 object-cover rounded-md"
								/>
								<div className="flex flex-col">
									<h3 className="text-white font-semibold text-lg">
										{world.name}
									</h3>
								</div>
							</div>
						))}
					</div>
				)}

				{tab === 'screenshots' && (
					<div className="w-full grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 sm:gap-6">
						{gameInfo?.screenshots.map((screenshot, index) => (
							<div
								key={index}
								className="bg-neutral-700 rounded-lg overflow-hidden shadow-lg hover:shadow-xl transition"
							>
								<img
									src={`data:image/png;base64,${screenshot.screenshot}`}
									alt={screenshot.name}
									className="w-full h-52 object-cover sm:h-60"
								/>
								<div className="p-3 sm:p-4">
									<p className="text-neutral-300 text-sm truncate">
										{screenshot.name}
									</p>
								</div>
							</div>
						))}
					</div>
				)}

				{tab === 'console' && (
					<div>

					</div>
				)}
			</div>
		</div>
	);
};
