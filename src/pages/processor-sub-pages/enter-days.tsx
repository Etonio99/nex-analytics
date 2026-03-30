import { useState } from 'react';
import Button from '../../components/button';
import ProcessorSubPage from './processor-sub-page';
import Input from '../../components/input';
import { BiCalendarPlus } from 'react-icons/bi';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import { useNotificationContext } from '../../components/contexts/notification-context';

const EnterDays = (props: ProcessSubPageProps) => {
  const { notify } = useNotificationContext();

  const getInitialDays = (): string => {
    if (props.advanceResult?.interrupt?.type === 'MISSING_DAYS') {
      if (props.advanceResult.interrupt.resolutionData?.type === 'NUMBER') {
        return props.advanceResult.interrupt.resolutionData.payload.toString();
      }
    }
    return '';
  };

  const [days, setDays] = useState<string>(getInitialDays());

  const continueProcess = async () => {
    if (!days) {
      notify('Missing Days', 'Please enter the number of days to continue');
      return;
    }

    try {
      if (!days.match(/^\d+$/)) {
        notify('Invalid Value', 'The "days" value must be a number');
        return;
      }
      const parsedDays = parseInt(days);
      if (Number.isNaN(parsedDays)) {
        notify('Invalid Value', 'The "days" value must be a number');
        return;
      }
      if (parsedDays > 60) {
        notify('Invalid value', 'The "days" value must be a number');
        return;
      }
      await props.appActions.updateProcessorData({ days: parsedDays });
      await props.appActions.advanceProcessor();
    } catch (error) {
      notify('Internal Error', 'Error while processing the "days" value');
      console.error(error);
    }
  };

  return (
    <ProcessorSubPage
      title="Enter Days"
      description="Enter the number of days forward you would like check for available slots."
      appActions={props.appActions}
    >
      <Input
        label="Days"
        placeholder="7"
        icon={<BiCalendarPlus />}
        value={days}
        onChange={(e) => setDays(e.target.value)}
      />
      <div className="mt-2 flex justify-end items-center gap-2">
        <Button
          label="Save"
          style="primary"
          onClick={continueProcess}
          disabled={!days}
        />
      </div>
    </ProcessorSubPage>
  );
};

export default EnterDays;
