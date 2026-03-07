'use client';

import { useEffect, useState } from 'react';
import CheckApiKey from './processor-sub-pages/check-api-key';
import LoadingIndicator from '../components/loading-indicator';
import { ProcessorAdvanceResult } from '../types/processor-advance-result';
import { useProcessor } from '../hooks/useProcessor';
import { ProcessStep } from '../types/processor-steps';
import EnterSubdomain from './processor-sub-pages/enter-subdomain';
import { ProcessorDataUpdate } from '../types/processor-data-update';
import { useAppState } from '../hooks/useAppState';
import { AppData } from '../types/app-data';

export type AppActions = {
  advanceProcessor: () => Promise<boolean>;
  updateProcessorData: (data: ProcessorDataUpdate) => Promise<boolean>;
  updateAppData: (data: AppData) => Promise<boolean>;
};

const Process = () => {
  const [advanceResult, setAdvanceResult] = useState<
    ProcessorAdvanceResult | undefined
  >(undefined);

  const { advanceProcessor, updateProcessorData } = useProcessor();
  const { updateAppData } = useAppState();

  const advance = async (): Promise<boolean> => {
    try {
      const response = await advanceProcessor();
      console.log(response);
      setAdvanceResult(response);
      return true;
    } catch (error) {
      console.error(error);
      return false;
    }
  };

  useEffect(() => {
    advance();
  }, []);

  const appActions: AppActions = {
    advanceProcessor: advance,
    updateProcessorData,
    updateAppData,
  };

  const getPage = (stepName: ProcessStep | undefined) => {
    if (!stepName) {
      return <LoadingIndicator />;
    }

    switch (stepName) {
      case 'CheckApiKey':
        return (
          <CheckApiKey appActions={appActions} advanceResult={advanceResult} />
        );
      case 'EnterSubdomain':
        return (
          <EnterSubdomain
            appActions={appActions}
            advanceResult={advanceResult}
          />
        );
    }
  };

  return getPage(advanceResult?.step);
};

export default Process;
