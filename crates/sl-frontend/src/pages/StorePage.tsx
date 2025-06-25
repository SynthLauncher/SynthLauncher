"use client";

import { Search } from "@/lib/types/store";
import { useEffect, useState } from "react";
import { CategorySelector, CategoryList, CategoryTrigger } from "@/components/ui/category-selector";
import { StoreCard } from "@/components/StoreCard";
import { Input } from "@/components/ui/input";
import { SearchIcon } from "lucide-react";
import { getStoreSearch } from "@/lib/commands/store";

export const StorePage = () => {
  const [data, setData] = useState<Search>();
  const [searchQuery, setSearchQuery] = useState("");
  const [category, setCategory] = useState("modpack");

  useEffect(() => {
    const fetchData = async () => {
      const search = await getStoreSearch(searchQuery, category);
      if (search) {
        setData(search);
      }
    };

    fetchData();
  }, [searchQuery, category]);

  return (
    <div className="p-6 w-full overflow-y-auto pb-20">
      <div className="flex flex-col gap-3">
        <CategorySelector defaultValue="modrinth">
          <CategoryList>
            <CategoryTrigger value="modrinth">Modrinth</CategoryTrigger>
            <CategoryTrigger value="curseforge">Curseforge</CategoryTrigger>
          </CategoryList>
        </CategorySelector>

        <CategorySelector defaultValue="modpack" onValueChange={(value) => setCategory(value)}>
          <CategoryList>
            <CategoryTrigger value="modpack">Modpacks</CategoryTrigger>
            <CategoryTrigger value="mod">Mods</CategoryTrigger>
            <CategoryTrigger value="shader">Shaders</CategoryTrigger>
            <CategoryTrigger value="resourcepack">Resource Packs</CategoryTrigger>
            <CategoryTrigger value="datapack">Data Packs</CategoryTrigger>
          </CategoryList>
        </CategorySelector>

        <Input icon={<SearchIcon className="w-4 h-4" />} placeholder="Search modpacks..."    value={searchQuery}
  onChange={(e) => setSearchQuery(e.target.value)} />

        {data?.hits.map((hit) => (
          <StoreCard 
            author={hit.author} 
            description={hit.description} 
            downloads={hit.downloads} 
            followers={hit.follows} 
            imageUrl={hit.icon_url ? hit.icon_url : ""} 
            name={hit.title} slug={hit.slug} key={hit.slug}  />
        ))}
      </div>
    </div>
  );
};
