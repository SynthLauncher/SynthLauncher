"use client";

import { Search } from "@/lib/types/store";
import { useEffect, useState } from "react";
import { CategorySelector, CategoryList, CategoryTrigger } from "@/components/ui/category-selector";
import { StoreCard } from "@/components/StoreCard";
import { StoreCardProps } from "@/lib/types/store";
import { Input } from "@/components/ui/input";
import { SearchIcon } from "lucide-react";
import { getStoreSearch } from "@/lib/commands/store";

export const StorePage = () => {
  const [data, setData] = useState<Search>();

  useEffect(() => {
    const fetchData = async () => {
      const search = await getStoreSearch();
      if (search) {
        setData(search); // make sure 'search' is of type Search[]
      }
    };

    fetchData();
  }, []);

  const exampleCard = {
    name: "Sodium",
    description: "The fastest and most compatible rendering optimization mod for Minecraft. Now available for both NeoForge and Fabric!",
    author: "jellysquid3",
    downloads: 56_690_000,
    followers: 24_600,
    imageUrl: "https://cdn.modrinth.com/data/AANobbMI/295862f4724dc3f78df3447ad6072b2dcd3ef0c9_96.webp",
    slug: "sodium",
  } satisfies StoreCardProps;

  return (
    <div className="p-6 w-full overflow-y-auto pb-20">
      <div className="flex flex-col gap-3">
        <CategorySelector defaultValue="modrinth">
          <CategoryList>
            <CategoryTrigger value="modrinth">Modrinth</CategoryTrigger>
            <CategoryTrigger value="curseforge">Curseforge</CategoryTrigger>
          </CategoryList>
        </CategorySelector>

        <CategorySelector defaultValue="modpacks">
          <CategoryList>
            <CategoryTrigger value="modpacks">Modpacks</CategoryTrigger>
            <CategoryTrigger value="mods">Mods</CategoryTrigger>
            <CategoryTrigger value="shaders">Shaders</CategoryTrigger>
            <CategoryTrigger value="resource-packs">Resource Packs</CategoryTrigger>
            <CategoryTrigger value="data-packs">Data Packs</CategoryTrigger>
          </CategoryList>
        </CategorySelector>

        <Input icon={<SearchIcon className="w-4 h-4" />} placeholder="Search modpacks..." />

        {data?.hits.map((hit) => (
          <StoreCard author={hit.author} description={hit.description} downloads={hit.downloads} followers={hit.follows} imageUrl={hit.icon_url} name={hit.title} slug={hit.slug} key={hit.slug}  />
        ))}
      </div>
    </div>
  );
};
