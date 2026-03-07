import { invoke } from '@tauri-apps/api/core';
import { AppData } from '../types/app-data';

export const useAppState = () => {
  const updateAppData = async (data: AppData): Promise<boolean> =>
    invoke('update_app_data', { data })
      .then(() => true)
      .catch(() => false);

  return {
    updateAppData,
  };
};
