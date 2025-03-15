'use client';

import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Sidebar } from '@/components/layout/sidebar';
import Banner from '@/components/layout/banner';
import Cards from '@/components/layout/cards';


export default function Home() {
  const [data, setData] = useState('');
  
  const handleClick = async () => {
    try {
      const message = await invoke('my_custom_command');
      setData(message as string);
    } catch (error) {
      console.error('Error invoking command:', error);
    }
  };
  
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
