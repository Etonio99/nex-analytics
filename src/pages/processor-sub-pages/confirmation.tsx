import Button from '../../components/button';
import ProcessorSubPage from './processor-sub-page';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import { interruptMessages } from '../../types/processor-interrupt';
import { snakeCaseToTitleCase } from '../../utils/string-helper';
import { BiEdit } from 'react-icons/bi';
import { ProcessStep } from '../../types/processor-steps';

const Confirmation = (props: ProcessSubPageProps) => {
  const continueProcess = async () => {
    await props.appActions.updateProcessorData({ confirmed: true });
    await props.appActions.advanceProcessor();
  };

  const confirmationData =
    props.advanceResult?.error?.resolutionData?.type === 'CONFIRMATION'
      ? props.advanceResult.error.resolutionData.payload
      : null;

  const getStepFromConfirmationData = (
    key: string
  ): ProcessStep | undefined => {
    switch (key) {
      case 'subdomain':
        return 'EnterSubdomain';
      case 'locations_count':
        return 'SelectLocations';
      case 'days':
        return 'EnterDays';
      case 'appointment_type_name':
        return 'EnterAppointmentTypeName';
    }
  };

  return (
    <ProcessorSubPage title="Confirmation">
      <p>
        Please confirm that all of the provided information is correct before
        proceeding.
      </p>
      <div className="border border-sandstone-300 rounded-lg overflow-hidden shadow shadow-sandstone-950/20 my-2">
        <ul>
          <li className="grid grid-cols-[1fr_1fr_32px] px-4 py-2 font-bold text-sandstone-400">
            <h2>Option</h2>
            <h2>Value</h2>
          </li>
          <hr className="border-sandstone-200" />
          {confirmationData &&
            Object.entries(confirmationData).map(([key, value]) => {
              const step = getStepFromConfirmationData(key);

              return (
                <li
                  key={key}
                  className="grid grid-cols-[1fr_1fr_32px] px-4 py-2 even:bg-sandstone-100"
                >
                  <p>{snakeCaseToTitleCase(key)}</p>
                  <p>{value}</p>
                  {step && (
                    <button
                      className="text-sandstone-400 grid place-items-center"
                      onClick={() => props.appActions.jumpToStep(step)}
                    >
                      <BiEdit />
                    </button>
                  )}
                </li>
              );
            })}
        </ul>
      </div>
      <div className="flex justify-end items-center gap-2">
        <Button label="Confirm" style="primary" onClick={continueProcess} />
        <Button label="Cancel" style="tertiary" />
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

export default Confirmation;
