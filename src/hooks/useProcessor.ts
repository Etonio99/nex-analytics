import { invoke } from '@tauri-apps/api/core';
import { ProcessorAdvanceResult } from '../types/processor-advance-result';

export const useProcessor = () => {
  const setProcessor = async (processorName: string): Promise<boolean> =>
    invoke('set_processor', {
      processorName,
    })
      .then(() => true)
      .catch(() => false);

  const advanceProcessor = async (): Promise<
    ProcessorAdvanceResult | undefined
  > =>
    invoke<ProcessorAdvanceResult>('advance_processor')
      .then((response) => response)
      .catch(() => undefined);

  const updateProcessorData = async (data: any): Promise<boolean> =>
    invoke('update_processor_data', data)
      .then(() => true)
      .catch(() => false);

  return {
    setProcessor,
    advanceProcessor,
    updateProcessorData,
  };
};
