'use client';

import { useEffect, useState } from 'react';
import CheckApiKey from './check-api-key';
import LoadingIndicator from '../components/loading-indicator';
import { ProcessorAdvanceResult } from '../types/processor-advance-result';
import { useProcessor } from '../hooks/useProcessor';
import { ProcessStep } from '../types/processor-steps';

const Process = () => {
  const [advanceResult, setAdvanceResult] = useState<
    ProcessorAdvanceResult | undefined
  >(undefined);

  const { advanceProcessor, updateProcessorData } = useProcessor();

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

  const update = async (data: any): Promise<boolean> =>
    updateProcessorData(data);

  useEffect(() => {
    advance();
  }, []);

  const getPage = (stepName: ProcessStep | undefined) => {
    if (!stepName) {
      return <LoadingIndicator />;
    }

    switch (stepName) {
      case 'CheckApiKey':
        return <CheckApiKey advance={advance} update={update} />;
    }
  };

  return getPage(advanceResult?.step);
};

export default Process;
