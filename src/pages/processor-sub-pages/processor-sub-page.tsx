import { BiX } from 'react-icons/bi';
import { useModalContext } from '../../components/contexts/modal-context';
import { AppActions } from '../process';
import React from 'react';

interface ProcessorSubPageProps {
  children: React.ReactNode;
  title: string;
  titleIcon?: React.ReactNode;
  description?: string;
  appActions: AppActions;
  hideCancelButton?: boolean;
}

const ProcessorSubPage = (props: ProcessorSubPageProps) => {
  const { confirm } = useModalContext();

  const cancelProcess = async () => {
    const confirmed = await confirm({
      title: "Are you sure you'd like to cancel?",
      description: 'You will have to start from the beginning if you leave.',
      cancelLabel: 'Nevermind',
      confirmLabel: "I'm sure",
    });

    if (!confirmed) {
      return;
    }

    await props.appActions.finish();
  };

  return (
    <div className="w-full">
      <h1 className="text-4xl font-bold mb-4 text-sandstone-300 flex items-center gap-2">
        {props.title}
        {props.titleIcon}
      </h1>
      {props.description && <p className="mb-3">{props.description}</p>}
      <div>{props.children}</div>
      {!props.hideCancelButton && (
        <button
          onClick={cancelProcess}
          className="absolute top-4 right-4 p-2 rounded-sm hover:bg-sandstone-100 text-sandstone-500 hover:text-sandstone-600"
        >
          <BiX size={18} />
        </button>
      )}
    </div>
  );
};

export default ProcessorSubPage;
