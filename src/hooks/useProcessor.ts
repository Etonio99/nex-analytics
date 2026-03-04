import { invoke } from '@tauri-apps/api/core';
import { useEffect } from 'react';

export const useProcessor = (autoAdvance: boolean) => {
  const setProcessor = async (processorName: string): Promise<boolean> =>
    invoke('set_processor', {
      processorName,
    })
      .then(() => true)
      .catch(() => false);

  const advanceProcessor = async (): Promise<boolean> =>
    invoke('advance_processor')
      .then(() => true)
      .catch(() => false);

  useEffect(() => {
    if (autoAdvance) {
      advanceProcessor();
    }
  }, []);

  return {
    setProcessor,
    advanceProcessor,
  };
};
