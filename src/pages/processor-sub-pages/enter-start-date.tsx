import { useState } from 'react';
import Button from '../../components/button';
import ProcessorSubPage from './processor-sub-page';
import Input from '../../components/input';
import { BiCalendar } from 'react-icons/bi';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import { useNotificationContext } from '../../components/contexts/notification-context';

const EnterStartDate = (props: ProcessSubPageProps) => {
  const { notify } = useNotificationContext();

  const getInitialStartDate = (): string => {
    if (props.advanceResult?.interrupt?.type === 'MISSING_START_DATE') {
      if (props.advanceResult.interrupt.resolutionData?.type === 'STRING') {
        return props.advanceResult.interrupt.resolutionData.payload.toString();
      }
    }
    return '';
  };

  const [startDate, setStartDate] = useState<string>(getInitialStartDate());

  const continueProcess = async () => {
    if (!startDate) {
      notify('Missing Start Date', 'Please enter a start date to continue');
      return;
    }

    await props.appActions.updateProcessorData({ start_date: startDate });
    await props.appActions.advanceProcessor();
  };

  return (
    <ProcessorSubPage
      title="Enter Start Date"
      description="Enter the date you would like start checking for available slots."
      appActions={props.appActions}
    >
      <Input
        label="Start Date"
        placeholder="7"
        icon={<BiCalendar />}
        value={startDate}
        type="date"
        onChange={(e) => setStartDate(e.target.value)}
      />
      <div className="mt-2 flex justify-end items-center gap-2">
        <Button
          label="Save"
          style="primary"
          onClick={continueProcess}
          disabled={!startDate}
        />
      </div>
    </ProcessorSubPage>
  );
};

export default EnterStartDate;
