import { useState } from 'react';
import Button from '../../components/button';
import ProcessorSubPage from './processor-sub-page';
import Input from '../../components/input';
import { BiCalendar } from 'react-icons/bi';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import { interruptMessages } from '../../types/processor-interrupt';

const EnterDays = (props: ProcessSubPageProps) => {
  const getInitialDays = (): string => {
    if (props.advanceResult?.error?.type === 'MISSING_DAYS') {
      if (props.advanceResult.error.resolutionData?.type === 'NUMBER') {
        return props.advanceResult.error.resolutionData.payload.toString();
      }
    }
    return '';
  };

  const [days, setDays] = useState<string>(getInitialDays());

  const continueProcess = async () => {
    if (!days) {
      return;
    }

    try {
      const parsedDays = parseInt(days);
      if (Number.isNaN(parsedDays)) {
        throw new Error('Days value is not a number');
      }
      if (parsedDays > 60) {
        throw new Error('Days can not be longer than 60');
      }
      await props.appActions.updateProcessorData({ days: parsedDays });
      await props.appActions.advanceProcessor();
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <ProcessorSubPage title="Enter Days">
      <Input
        label="Days"
        placeholder="7"
        icon={<BiCalendar />}
        value={days}
        onChange={(e) => setDays(e.target.value)}
      />
      <div className="mt-2 flex justify-end items-center gap-2">
        <Button label="Save" style="primary" onClick={continueProcess} />
      </div>
      {props.advanceResult && (
        <p className="text-red-400 w-full text-center">
          {
            interruptMessages[
              props.advanceResult.error?.type as keyof typeof interruptMessages
            ]
          }
        </p>
      )}
    </ProcessorSubPage>
  );
};

export default EnterDays;
