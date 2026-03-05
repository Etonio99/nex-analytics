import { ProcessorError } from './processor-error';
import { ProcessStep } from './processor-steps';

export type ProcessorAdvanceResult = {
  step: ProcessStep;
  error: ProcessorError | null;
};
