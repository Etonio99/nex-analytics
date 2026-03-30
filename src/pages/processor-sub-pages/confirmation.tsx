import Button from '../../components/button';
import ProcessorSubPage from './processor-sub-page';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import { snakeCaseToTitleCase } from '../../utils/string-helper';
import { BiEdit } from 'react-icons/bi';
import { ProcessStep } from '../../types/processor-steps';
import { useModalContext } from '../../components/contexts/modal-context';

const Confirmation = (props: ProcessSubPageProps) => {
  const { confirm } = useModalContext();

  const continueProcess = async () => {
    await props.appActions.updateProcessorData({ confirmed: true });
    await props.appActions.advanceProcessor();
  };

  const confirmationData =
    props.advanceResult?.interrupt?.resolutionData?.type === 'CONFIRMATION'
      ? props.advanceResult.interrupt.resolutionData.payload
      : null;

  const getStepFromConfirmationData = (
    key: string
  ): ProcessStep | undefined => {
    switch (key) {
      case 'subdomain':
        return 'EnterSubdomain';
      case 'locations_count':
        return 'SelectLocations';
      case 'start_date':
        return 'EnterStartDate';
      case 'days':
        return 'EnterDays';
      case 'appointment_type_name':
        return 'EnterAppointmentTypeName';
    }
  };

  const cancel = async () => {
    const confirmed = await confirm({
      title: "Are you sure you'd like to cancel?",
      description: 'You will have to start from the beginning if you leave.',
      cancelLabel: 'Nevermind',
      confirmLabel: "I'm sure",
    });

    if (!confirmed) {
      return;
    }

    props.appActions.finish();
  };

  return (
    <ProcessorSubPage title="Confirmation" appActions={props.appActions}>
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
        <Button label="Cancel" style="tertiary" onClick={cancel} />
        <Button label="Confirm" style="primary" onClick={continueProcess} />
      </div>
    </ProcessorSubPage>
  );
};

export default Confirmation;
