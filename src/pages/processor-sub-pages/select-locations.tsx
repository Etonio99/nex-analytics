import { Suspense } from 'react';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import ProcessorSubPage from './processor-sub-page';
import LocationSelect from './components/location-select';
import Button from '../../components/button';

const SelectLocations = (props: ProcessSubPageProps) => {
  const continueProcess = async () => {
    // await props.appActions.updateAppData({ subdomain: subdomainInput });
    props.appActions.advanceProcessor();
  };

  return (
    <ProcessorSubPage title="Select Locations">
      <Suspense fallback={<p>Loading...</p>}>
        <LocationSelect />
        <Button label="Save" style="primary" onClick={continueProcess} />
      </Suspense>
    </ProcessorSubPage>
  );
};

export default SelectLocations;
