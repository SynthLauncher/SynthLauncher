import { useEffect, useState } from 'react';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';
import { getMinecraftVersions } from '@/lib/commands/minecraft';
import { ArrowUpNarrowWide, Box, Plus, Upload, X } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Label } from '@/components/ui/label';

// TODO: Improve this
export const CreateInstanceDialog = ({
	open,
	onOpenChange,
	onCreate,
}: {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	onCreate: (name: string, version: string, loader: string) => void;
}) => {
	const [name, setName] = useState('');
	const [version, setVersion] = useState('');
	const [loader, setLoader] = useState('');
	const [minecraftVersions, setMinecraftVersions] = useState<string[]>([]);

	useEffect(() => {
		const fetch = async () => {
			let versions = await getMinecraftVersions();
			setMinecraftVersions(versions);

			if (versions.length > 0 && !version) {
				setVersion(versions[0]);
			}

			if (!loader) {
				setLoader('Vanilla');
			}
		};

		fetch();
	}, []);

	return (
		<Dialog open={open} onOpenChange={onOpenChange}>
			<DialogContent className="sm:max-w-[475px] bg-[#1E2128] border-[#2c3039] border-2">
				<DialogHeader>
					<DialogTitle className="text-white">Create New Instance</DialogTitle>
					<DialogDescription className="text-gray-300">
						Configure your new Minecraft instance.
					</DialogDescription>
				</DialogHeader>

				<div className="grid gap-4 py-4">
					<div className="grid gap-2">
						<Label htmlFor="name" className="text-sm font-medium text-gray-200">
							Instance Name
						</Label>
						<Input
							id="name"
							className="border-0 bg-[#2b3136] text-white placeholder:text-[#9ca5a8]"
							placeholder="My new instance"
							value={name}
							onChange={(e) => setName(e.target.value)}
						/>
					</div>

					<div className="grid gap-2">
						<Label
							htmlFor="version"
							className="text-sm font-medium text-gray-200"
						>
							Minecraft Version
						</Label>
						<select
							id="version"
							className="appearance-none flex h-9 w-full rounded-md bg-[#2b3136] px-3 py-1 text-sm text-gray-200 justify-between outline-none"
							onChange={(e) => setVersion(e.target.value)}
							value={version}
						>
							{minecraftVersions.map((version) => (
								<option key={version} value={version}>{version}</option>
							))}
						</select>
					</div>

					<div className="grid gap-2">
						<Label
							htmlFor="modloader"
							className="text-sm font-medium text-gray-200"
						>
							Mod Loader
						</Label>
						<select
							id="modloader"
							className="appearance-none flex h-9 w-full rounded-md bg-[#2b3136] px-3 py-1 text-sm text-gray-200 outline-none"
							onChange={(e) => setLoader(e.target.value)}
							value={loader}
						>
							<option value="Vanilla">Vanilla</option>
							<option value="NeoForge">Neoforge</option>
							<option value="Forge">Forge</option>
							<option value="Fabric">Fabric</option>
							<option value="Quilt">Quilt</option>
						</select>
					</div>

					<div className="grid gap-2">
						<Label htmlFor="icon" className="text-sm font-medium text-gray-200">
							Instance Icon
						</Label>
						<div className="flex items-center gap-4">
							<div className="w-22 h-22 bg-[#2b3136] rounded-lg flex items-center justify-center">
								<Box className="text-neutral-300" width={68} height={68} />
							</div>
							<div className="flex flex-col justify-center gap-2">
								<button className="px-5 py-[6px] bg-[#2b3136]  hover:bg-gray-600 text-gray-200 rounded-lg transition-colors flex gap-[6px] items-center">
									<Upload width={22} height={22} />
									Choose Icon
								</button>
								<button className="px-5 py-[6px] hover:bg-[#d14646] bg-[#f34b4b] disabled:bg-[#d14646]/50 disabled:text-neutral-800 text-neutral-900 rounded-lg transition-colors flex gap-[6px] items-center">
									<X width={22} height={22} />
									Remove Icon
								</button>
							</div>
						</div>
					</div>
				</div>

				<DialogFooter>
					<Button
						onClick={() => onOpenChange(false)}
						className="px-4 py-4 bg-[#2b3136] hover:bg-gray-600 text-gray-200 rounded-lg transition-colors flex items-center gap-1"
					>
						<ArrowUpNarrowWide width={24} height={24} />
						<span>Show Advanced</span>
					</Button>
					<Button
						onClick={() => onOpenChange(false)}
						className="px-5 py-4 bg-[#2b3136] hover:bg-gray-600 text-gray-200 rounded-lg transition-colors flex items-center gap-1"
					>
						<X width={24} height={24} />
						<span>Cancel</span>
					</Button>
					<Button
						onClick={() => {
							onOpenChange(false);
							onCreate(name, version, loader);
						}}
						className="px-6 py-4 bg-[#f56241] hover:bg-[#f56241]/50 text-white rounded-lg transition-colors flex items-center gap-1"
					>
						<Plus width={24} height={24} />
						<span>Create Instance</span>
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
};
