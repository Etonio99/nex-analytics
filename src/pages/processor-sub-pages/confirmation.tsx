import { snakeCaseToTitleCase } from '../../utils/string-helper';
import { BiEdit } from 'react-icons/bi';
import { ProcessStep } from '../../types/processor-steps';
import { DataConfirmation } from '../../types/data-confirmation';

interface ConfirmationProps {
  confirmationData: DataConfirmation;
  jumpToStep: (step: ProcessStep) => Promise<boolean>;
}

const Confirmation = (props: ConfirmationProps) => {
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

  // const cancel = async () => {
  //   const confirmed = await confirm({
  //     title: "Are you sure you'd like to cancel?",
  //     description: 'You will have to start from the beginning if you leave.',
  //     cancelLabel: 'Nevermind',
  //     confirmLabel: "I'm sure",
  //   });

  //   if (!confirmed) {
  //     return;
  //   }

  //   props.appActions.finish();
  // };

  return (
    <div className="border border-sandstone-300 rounded-lg overflow-hidden shadow shadow-sandstone-950/20 my-2">
      <ul>
        <li className="grid grid-cols-[1fr_1fr_32px] px-4 py-2 font-bold text-sandstone-400">
          <h2>Option</h2>
          <h2>Value</h2>
        </li>
        <hr className="border-sandstone-200" />
        {Object.entries(props.confirmationData).map(([key, value]) => {
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
                  onClick={() => props.jumpToStep(step)}
                >
                  <BiEdit />
                </button>
              )}
            </li>
          );
        })}
      </ul>
    </div>
  );
};

export default Confirmation;
