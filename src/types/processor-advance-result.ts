import { ProcessorInterrupt } from './processor-interrupt';
import { ProcessStep } from './processor-steps';

export type ProcessorAdvanceResult = {
  step: ProcessStep;
  error: ProcessorInterrupt | null;
};
