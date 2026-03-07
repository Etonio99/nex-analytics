import { invoke } from '@tauri-apps/api/core';

export const useController = () => {
  const getLocations = async (processorName: string): Promise<boolean> =>
    invoke('set_processor', {
      processorName,
    })
      .then(() => true)
      .catch(() => false);

  return {
    getLocations,
  };
};
