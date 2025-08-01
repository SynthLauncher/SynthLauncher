import { NewsCarousel } from "@/components/ui/news-carousel";
import { InstanceCard } from "./instances-page";
import { Instance } from "@/lib/types/instances";
import { useEffect, useState } from "react";
import { getInstances } from "@/lib/commands/instances";

export const HomePage = () => {
	const newsItems = [
		{
			id: 1,
			imageUrl: "https://i.imgur.com/joXSHn2.jpg",
			title: "News Item 1",
			description: "This is a description for news item 1.",
			url: "https://example.com/news1",
		},
		{
			id: 2,
			imageUrl: "https://i.imgur.com/joXSHn2.jpg",
			title: "News Item 2",
			description: "This is a description for news item 2.",
			url: "https://example.com/news2",
		},
		{
			id: 3,
			imageUrl: "https://i.imgur.com/joXSHn2.jpg",
			title: "News Item 3",
			description: "This is a description for news item 3.",
			url: "https://example.com/news3",
		},
		{
			id: 4,
			imageUrl: "https://i.imgur.com/joXSHn2.jpg",
			title: "News Item 4",
			description: "This is a description for news item 4.",
			url: "https://example.com/news4",
		},
	];

	const [instances, setInstances] = useState<Instance[]>([]);

	const fetchInstances = async () => {
		const all = await getInstances();
		setInstances(all ?? []);
	};

	useEffect(() => {
		fetchInstances();
	}, []);

	return <div>
		<div className="flex flex-col gap-5">
			<div>
				<h1 className="text-2xl mb-2">News</h1>
				<NewsCarousel autoplayDelay={10_000} items={newsItems} />
			</div>

			<div>
				<h1 className="text-2xl mb-2">Recently Used Instances</h1>
				<div className="flex gap-3">
					{instances.map((inst) => (
						<InstanceCard key={inst.name} {...inst} />
					))}
				</div>
			</div>
		</div>
	</div>;
};
