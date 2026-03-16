import { useState } from 'react';
import Button from '../../components/button';
import useApiKey from '../../hooks/useApiKey';
import ProcessorSubPage from './processor-sub-page';
import Input from '../../components/input';
import { BiSolidKey } from 'react-icons/bi';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import { interruptMessages } from '../../types/processor-interrupt';

const CheckApiKey = (props: ProcessSubPageProps) => {
  const [apiKeyInput, setApiKeyInput] = useState<string>('');

  const { setApiKey } = useApiKey();

  const continueProcess = async () => {
    if (!apiKeyInput) {
      return;
    }

    await setApiKey(apiKeyInput);
    await props.appActions.advanceProcessor();
  };

  return (
    <ProcessorSubPage title="Check API Key">
      <Input
        label="API Key"
        placeholder="eCWxyomJxd56bv8.xPL7gwq..."
        icon={<BiSolidKey />}
        value={apiKeyInput}
        onChange={(e) => setApiKeyInput(e.target.value)}
      />
      <div className="mt-2 flex justify-end items-center gap-2">
        <p className="text-sandstone-300 italic">
          Your API key will be stored securely on your machine for reuse.
        </p>
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

export default CheckApiKey;
