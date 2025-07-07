import { invoke } from '@tauri-apps/api/core';

export interface ElyByTexture {
  url: string;
  metadata?: {
    model?: string;
  };
}

export interface ElyByTextures {
  skin?: ElyByTexture;
  cape?: ElyByTexture;
}

export const getSkinUrl = async (nickname: string): Promise<string | null> =>
  invoke<string | null>('get_skin_url', { nickname });

export const getCapeUrl = async (nickname: string): Promise<string | null> =>
  invoke<string | null>('get_cape_url', { nickname });

export const getTextures = async (nickname: string): Promise<ElyByTextures | null> =>
  invoke<ElyByTextures | null>('get_textures', { nickname }); 