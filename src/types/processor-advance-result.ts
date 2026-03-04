import { ProcessStep } from './processor-steps';

export type ProcessorAdvanceResult = {
  step: ProcessStep;
  error: string | null;
};
