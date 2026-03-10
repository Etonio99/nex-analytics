'use client';

import { useEffect, useRef, useState } from 'react';
import CheckApiKey from './processor-sub-pages/check-api-key';
import LoadingIndicator from '../components/loading-indicator';
import { ProcessorAdvanceResult } from '../types/processor-advance-result';
import { useProcessor } from '../hooks/useProcessor';
import { ProcessStep } from '../types/processor-steps';
import EnterSubdomain from './processor-sub-pages/enter-subdomain';
import { ProcessorDataUpdate } from '../types/processor-data-update';
import { useAppState } from '../hooks/useAppState';
import { AppData } from '../types/app-data';
import SelectLocations from './processor-sub-pages/select-locations';
import EnterDays from './processor-sub-pages/enter-days';
import EnterAppointmentTypeName from './processor-sub-pages/enter-appointment-type-name';
import Confirmation from './processor-sub-pages/confirmation';

export type AppActions = {
  advanceProcessor: () => Promise<boolean>;
  updateProcessorData: (data: ProcessorDataUpdate) => Promise<boolean>;
  updateAppData: (data: AppData) => Promise<boolean>;
};

const Process = () => {
  const [advanceResult, setAdvanceResult] = useState<
    ProcessorAdvanceResult | undefined
  >(undefined);

  const startedProcess = useRef(false);

  const { advanceProcessor, updateProcessorData } = useProcessor();
  const { updateAppData } = useAppState();

  const advance = async (): Promise<boolean> => {
    try {
      setAdvanceResult(undefined);
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
    if (startedProcess.current) {
      return;
    }
    advance();
    startedProcess.current = true;
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

    console.log(stepName);

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
      case 'SelectLocations':
        return (
          <SelectLocations
            appActions={appActions}
            advanceResult={advanceResult}
          />
        );
      case 'EnterDays':
        return (
          <EnterDays appActions={appActions} advanceResult={advanceResult} />
        );
      case 'EnterAppointmentTypeName':
        return (
          <EnterAppointmentTypeName
            appActions={appActions}
            advanceResult={advanceResult}
          />
        );
      case 'Confirmation':
        return (
          <Confirmation appActions={appActions} advanceResult={advanceResult} />
        );
    }
  };

  return (
    <div className="h-full max-w-2xl m-auto grid place-items-center">
      {getPage(advanceResult?.step)}
    </div>
  );
};

export default Process;
