import { Progress } from '@/components/ui/progress';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

export const HomePage = () => {
	const [state, setState] = useState<number>(0);

	useEffect(() => {
		const unlisten = listen('download-progress', (event) => {
			setState(event.payload as number);
		});

		return () => {
			unlisten.then((fn) => fn());
		};
	}, []);

	const test_progress = async () => {
		await invoke('test_progress');
	};

	return (
		<div className="p-6 pb-12 w-full overflow-auto">
			<div className="flex flex-col gap-2 mb-8">
				<h1 className="text-white text-xl">{state?.toFixed(2)}</h1>
				<Progress key={state} value={state} className="w-[60%] bg-white" />
				<button className="bg-white px-4 py-2 w-2xs" onClick={test_progress}>
					Click
				</button>
			</div>
		</div>
	);
};
