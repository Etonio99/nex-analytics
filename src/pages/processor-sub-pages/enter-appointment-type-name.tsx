import { useState } from 'react';
import Button from '../../components/button';
import ProcessorSubPage from './processor-sub-page';
import Input from '../../components/input';
import { BiRename } from 'react-icons/bi';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import { interruptMessages } from '../../types/processor-interrupt';

const EnterAppointmentTypeName = (props: ProcessSubPageProps) => {
  const getInitialAppointmentTypeName = (): string => {
    if (props.advanceResult?.error?.type === 'MISSING_APPOINTMENT_TYPE_NAME') {
      if (props.advanceResult.error.resolutionData?.type === 'STRING') {
        return props.advanceResult.error.resolutionData.payload;
      }
    }
    return '';
  };

  const [appointmentTypeName, setAppointmentTypeName] = useState<string>(
    getInitialAppointmentTypeName()
  );

  const continueProcess = async () => {
    if (!appointmentTypeName) {
      return;
    }

    await props.appActions.updateProcessorData({
      appointment_type_name: appointmentTypeName,
    });
    await props.appActions.advanceProcessor();
  };

  return (
    <ProcessorSubPage title="Enter Appointment Type Name">
      <Input
        label="Appointment Type Name"
        placeholder="New Patient Cleaning"
        icon={<BiRename />}
        value={appointmentTypeName}
        onChange={(e) => setAppointmentTypeName(e.target.value)}
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

export default EnterAppointmentTypeName;
