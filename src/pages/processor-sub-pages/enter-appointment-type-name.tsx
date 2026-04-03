import { useState } from 'react';
import Button from '../../components/button';
import ProcessorSubPage from './processor-sub-page';
import Input from '../../components/input';
import { BiRename } from 'react-icons/bi';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import { useNotificationContext } from '../../components/contexts/notification-context';

const EnterAppointmentTypeName = (props: ProcessSubPageProps) => {
  const { notify } = useNotificationContext();

  const getInitialAppointmentTypeName = (): string => {
    if (
      props.advanceResult?.interrupt?.type === 'MISSING_APPOINTMENT_TYPE_NAME'
    ) {
      if (props.advanceResult.interrupt.resolutionData?.type === 'STRING') {
        return props.advanceResult.interrupt.resolutionData.payload;
      }
    }
    return '';
  };

  const [appointmentTypeName, setAppointmentTypeName] = useState<string>(
    getInitialAppointmentTypeName()
  );

  const continueProcess = async () => {
    if (!appointmentTypeName) {
      notify(
        'Missing Appointment Type Name',
        'Please enter an appointment type name to continue'
      );
      return;
    }

    await props.appActions.updateProcessorData({
      appointment_type_name: appointmentTypeName,
    });
    await props.appActions.advanceProcessor();
  };

  return (
    <ProcessorSubPage
      title="Enter Appointment Type Name"
      description="Enter the name of the appointment type exactly as it appears in your NexHealth system."
      appActions={props.appActions}
    >
      <Input
        label="Appointment Type Name"
        placeholder="New Patient Cleaning"
        icon={<BiRename />}
        value={appointmentTypeName}
        onChange={(e) => setAppointmentTypeName(e.target.value)}
      />
      <div className="mt-2 flex justify-end items-center gap-2">
        <Button
          label="Save"
          style="primary"
          onClick={continueProcess}
          disabled={!appointmentTypeName}
        />
      </div>
    </ProcessorSubPage>
  );
};

export default EnterAppointmentTypeName;
