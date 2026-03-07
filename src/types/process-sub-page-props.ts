import { AppActions } from '../pages/process';
import { ProcessorAdvanceResult } from './processor-advance-result';

export interface ProcessSubPageProps {
  appActions: AppActions;
  advanceResult: ProcessorAdvanceResult | undefined;
}
