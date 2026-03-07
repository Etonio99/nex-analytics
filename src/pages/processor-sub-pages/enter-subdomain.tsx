import { useState } from 'react';
import Button from '../../components/button';
import ProcessorSubPage from './processor-sub-page';
import Input from '../../components/input';
import { BiBuildings } from 'react-icons/bi';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import { errorMessages } from '../../types/processor-error';

const EnterSubdomain = (props: ProcessSubPageProps) => {
  const [subdomainInput, setSubdomainInput] = useState<string>('');

  const continueProcess = async () => {
    if (!subdomainInput) {
      return;
    }

    await props.appActions.updateAppData({ subdomain: subdomainInput });
    props.appActions.advanceProcessor();
  };

  return (
    <ProcessorSubPage title="Enter Subdomain">
      <Input
        label="Subdomain"
        placeholder="my-institution-subdomain"
        icon={<BiBuildings />}
        value={subdomainInput}
        onChange={(e) => setSubdomainInput(e.target.value)}
      />
      <div className="mt-2 flex justify-end items-center gap-2">
        <Button label="Save" style="primary" onClick={continueProcess} />
      </div>
      {props.advanceResult && (
        <p className="text-red-400 w-full text-center">
          {
            errorMessages[
              props.advanceResult.error?.type as keyof typeof errorMessages
            ]
          }
        </p>
      )}
    </ProcessorSubPage>
  );
};

export default EnterSubdomain;
