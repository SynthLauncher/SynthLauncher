'use client';

import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button } from '@/components/ui/button';

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
    <div className="h-screen w-full bg-gray-900 text-white">

    </div>
  );
}
