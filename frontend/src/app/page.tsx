'use client';

import { Sidebar } from '@/components/layout/sidebar';
import Banner from '@/components/layout/banner';
import Cards from '@/components/layout/cards';

export default function Home() {
  return (
    <div className="flex">
      <Sidebar />
      <div className="flex-1">
        <Banner />
        <Cards />
      </div>
    </div>
  );
}
