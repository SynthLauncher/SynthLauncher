"use client";

import { getStoreSearch } from "@/lib/commands/store";
import { Search } from "@/lib/types/store";
import { useEffect, useState } from "react";
import { CategorySelector, CategoryList, CategoryTrigger } from "@/components/ui/category-selector";
import { StoreCard } from "@/components/StoreCard";
import { Input } from "@/components/ui/input";
import { SearchIcon } from "lucide-react";

export const StorePage = () => {

  return (
    <div className="p-6 w-full overflow-auto pb-20">
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

        <StoreCard slug="hi" />
        <StoreCard slug="hi" />
        <StoreCard slug="hi" />
        <StoreCard slug="hi" />
      </div>
    </div>
  );
};
