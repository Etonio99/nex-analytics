import { ProcessorAdvanceResult } from './processor-advance-result';

export interface ProcessSubPageProps {
  advance: () => Promise<boolean>;
  update: (data: never) => Promise<boolean>;
  advanceResult: ProcessorAdvanceResult | undefined;
}
