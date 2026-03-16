import { invoke } from '@tauri-apps/api/core';
import { ProcessorAdvanceResult } from '../types/processor-advance-result';
import { ProcessorDataUpdate } from '../types/processor-data-update';
import { ProcessStep } from '../types/processor-steps';

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

  const updateProcessorData = async (
    data: ProcessorDataUpdate
  ): Promise<boolean> =>
    invoke('update_processor_data', { data })
      .then(() => true)
      .catch(() => false);

  const jumpToStep = async (
    step: ProcessStep
  ): Promise<ProcessorAdvanceResult | undefined> =>
    invoke<ProcessorAdvanceResult>('jump_to_step', {
      step,
    })
      .then((response) => response)
      .catch(() => undefined);

  return {
    setProcessor,
    advanceProcessor,
    updateProcessorData,
    jumpToStep,
  };
};
