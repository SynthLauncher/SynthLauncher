'use client';

import Banner from "@/components/layout/banner";
import { Sidebar } from "@/components/layout/sidebar";

export default function Home() {
  return (
    <div className="flex">
      <Sidebar />
      <div className="flex=1">
        <Banner />
      </div>
    </div>
  );
}
