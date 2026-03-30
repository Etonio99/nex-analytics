'use client';

import { useEffect, useRef, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import CheckApiKey from './processor-sub-pages/check-api-key';
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
import Loading from './processor-sub-pages/loading';
import Complete from './processor-sub-pages/complete';
import { useNotificationContext } from '../components/contexts/notification-context';
import EnterStartDate from './processor-sub-pages/enter-start-date';

interface ProcessProps {
  navigate: (page: string) => void;
}

export type AppActions = {
  advanceProcessor: () => Promise<boolean>;
  updateProcessorData: (data: ProcessorDataUpdate) => Promise<boolean>;
  updateAppData: (data: AppData) => Promise<boolean>;
  jumpToStep: (step: ProcessStep) => Promise<boolean>;
  finish: () => Promise<void>;
};

const Process = (props: ProcessProps) => {
  const { notify } = useNotificationContext();

  const [advanceResult, setAdvanceResult] = useState<
    ProcessorAdvanceResult | undefined
  >(undefined);
  const [progressMessage, setProgressMessage] = useState('');

  const startedProcess = useRef(false);

  const { advanceProcessor, clearProcessor, updateProcessorData, jumpToStep } =
    useProcessor();
  const { updateAppData } = useAppState();

  const advance = async (): Promise<boolean> => {
    try {
      setAdvanceResult(undefined);
      const response = await advanceProcessor();
      console.log(response);
      setAdvanceResult(response);
      handleAdvanceResult(response);
      return true;
    } catch (error) {
      console.error(error);
      return false;
    }
  };

  const handleAdvanceResult = (result: ProcessorAdvanceResult | undefined) => {
    const interrupt = result?.interrupt;
    if (!interrupt) return;

    if (
      interrupt.type === 'PERMISSION_DENIED' &&
      interrupt.resolutionData?.type === 'STRING' &&
      interrupt.resolutionData.payload === 'subdomain'
    ) {
      notify(
        'Permission Denied',
        'You do not have permission to access this subdomain'
      );
      jump('EnterSubdomain');
    }
  };

  const jump = async (step: ProcessStep): Promise<boolean> => {
    try {
      setAdvanceResult(undefined);
      const response = await jumpToStep(step);
      console.log(response);
      setAdvanceResult(response);
      return true;
    } catch (error) {
      console.error(error);
      return false;
    }
  };

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    listen<string>('progress', (event) => {
      setProgressMessage(event.payload);
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      unlisten?.();
    };
  }, []);

  useEffect(() => {
    if (startedProcess.current) {
      return;
    }
    advance();
    startedProcess.current = true;
  }, []);

  const finish = async (): Promise<void> => {
    await clearProcessor();
    props.navigate('home');
  };

  const appActions: AppActions = {
    advanceProcessor: advance,
    updateProcessorData,
    updateAppData,
    jumpToStep: jump,
    finish,
  };

  const getPage = (stepName: ProcessStep | undefined) => {
    if (!stepName) {
      return <Loading message={progressMessage} />;
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
      case 'SelectLocations':
        return (
          <SelectLocations
            appActions={appActions}
            advanceResult={advanceResult}
          />
        );
      case 'EnterStartDate':
        return (
          <EnterStartDate
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
      case 'Complete':
        return (
          <Complete appActions={appActions} advanceResult={advanceResult} />
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
