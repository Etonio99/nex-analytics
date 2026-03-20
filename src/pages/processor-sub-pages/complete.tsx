import ProcessorSubPage from './processor-sub-page';
import useFileSystem from '../../hooks/useFileSystem';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import Button from '../../components/button';

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
    <ProcessorSubPage title="Process Complete">
      {path && (
        <>
          <p>
            Your data has successully been obtained and was saved to the
            following location:
          </p>
          <button
            onClick={() => revealFileOrDirectory(path)}
            className="text-green-500"
          >
            {path}
          </button>
        </>
      )}
      <Button label="Done" onClick={finish} style="primary" />
    </ProcessorSubPage>
  );
};

export default Complete;
