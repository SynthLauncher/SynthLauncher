import { getInstances, launchInstance } from "@/lib/commands/instances";
import { Instance } from "@/lib/types/instances";
import { Blocks, Play, Settings } from "lucide-react";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

export const InstancePage = () => {
	const { name } = useParams<{ name: string }>();
	const [instance, setInstance] = useState<Instance>();
	const [tab, setTab] = useState<"content" | "logs" | "saves" | "screenshots" | "console">("content");
	const [isRunning, setIsRunning] = useState(false);

	useEffect(() => {
		const fetchAll = async () => {
			let instances = await getInstances();
			const match = instances?.find(inst => inst.name === name);
			setInstance(match);
		};
		fetchAll();
	}, []);

	if (!instance) {
		return (
			<div className="p-8 text-center text-lg text-red-400 font-medium">
				Instance not found.
			</div>
		);
	}

	return (
		<div className="w-full p-8 min-h-screen text-white">
			<div className="flex items-center justify-between bg-neutral-800 rounded-2xl p-6 shadow-lg">
				<div className="flex gap-6 items-center">
					<div className="w-24 h-24 bg-neutral-700 rounded-xl flex justify-center items-center">
						<Blocks className="w-14 h-14 text-blue-400" />
					</div>

					<div className="flex flex-col gap-2">
						<h1 className="text-3xl font-bold">{instance.name}</h1>
						<div className="inline-block bg-blue-500/20 text-blue-400 text-sm font-semibold px-3 py-1 rounded-full w-fit">
							{instance.mod_loader}
						</div>
					</div>
				</div>

				<div className="flex gap-2 items-center">
					<button
						className="bg-green-500 hover:bg-green-600 text-black font-semibold px-5 h-10 rounded-md flex items-center gap-2 shadow transition disabled:opacity-50 disabled:cursor-not-allowed"
						onClick={async () => {
							setIsRunning(true);
							try {
								await launchInstance(instance.name);
							} finally {
								setIsRunning(false);
							}
						}}
						disabled={isRunning}
					>
						<Play className="w-5 h-5" />
						<span>{isRunning ? "Running..." : "Play"}</span>
					</button>

					<button className="w-10 h-10 flex items-center justify-center bg-neutral-700 hover:bg-neutral-600 rounded-md shadow transition">
						<Settings className="w-5 h-5 text-white" />
					</button>
				</div>
			</div>

			<div className="mt-6 border-b border-neutral-700 flex gap-4">
				{(["content", "logs", "saves", "screenshots", "console"] as const).map((t) => (
					<button
						key={t}
						onClick={() => setTab(t)}
						className={`capitalize px-4 py-2 font-medium border-b-2 ${tab === t
							? "text-blue-400 border-blue-400"
							: "text-neutral-400 border-transparent hover:text-white"
							} transition`}
					>
						{t}
					</button>
					))}
			</div>

			<div className="mt-6 p-6 bg-neutral-800 rounded-xl text-neutral-300 text-lg min-h-[200px] flex items-center justify-center">
				{tab === "content" && (
					<div className="text-center">
						<p className="text-white mb-4">You haven’t added any content to this instance yet.</p>
						<button className="bg-neutral-700 hover:bg-neutral-600 px-4 py-2 rounded-md text-white font-medium transition">
							+ Install content
						</button>
					</div>
				)}
				{tab === "logs" && <></>}
				{tab === "saves" && <></>}
				{tab === "screenshots" && <></>}
				{tab === "console" && <></>}
			</div>
		</div>
	);
};
