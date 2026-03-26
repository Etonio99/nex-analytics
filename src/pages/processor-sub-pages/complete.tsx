import ProcessorSubPage from './processor-sub-page';
import useFileSystem from '../../hooks/useFileSystem';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import Button from '../../components/button';
import { BiCheckDouble, BiLinkExternal } from 'react-icons/bi';

const Complete = (props: ProcessSubPageProps) => {
  const { revealFileOrDirectory } = useFileSystem();

  const finish = async () => {
    await props.appActions.finish();
  };

  const path =
    props.advanceResult?.interrupt?.resolutionData?.type === 'STRING'
      ? props.advanceResult.interrupt.resolutionData.payload
      : undefined;

  return (
    <ProcessorSubPage
      title="Process Complete"
      titleIcon={<BiCheckDouble size={42} />}
      appActions={props.appActions}
      hideCancelButton
    >
      {path && (
        <>
          <p>
            Your analytics have successully been obtained and were saved to the
            following location:
          </p>
          <button
            onClick={() => revealFileOrDirectory(path)}
            className="text-green-500 text-left"
          >
            {path}{' '}
            <span className="inline-block text-sandstone-300">
              <BiLinkExternal />
            </span>
          </button>
        </>
      )}
      <div className="w-fit mx-auto mt-4">
        <Button label="Finish" onClick={finish} style="primary" />
      </div>
    </ProcessorSubPage>
  );
};

export default Complete;
