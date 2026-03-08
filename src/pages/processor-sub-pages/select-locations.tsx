import { Suspense, useState } from 'react';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import ProcessorSubPage from './processor-sub-page';
import LocationSelect from './components/location-select';
import Button from '../../components/button';
import LoadingIndicator from '../../components/loading-indicator';

const SelectLocations = (props: ProcessSubPageProps) => {
  const [locationSelection, setLocationSelection] = useState<
    Record<string, boolean>
  >({});

  const continueProcess = async () => {
    // await props.appActions.updateAppData({ subdomain: subdomainInput });
    props.appActions.advanceProcessor();
  };

  return (
    <ProcessorSubPage title="Select Locations">
      <Suspense fallback={<LoadingIndicator />}>
        <LocationSelect
          value={locationSelection}
          onChange={setLocationSelection}
        />
        <Button label="Save" style="primary" onClick={continueProcess} />
      </Suspense>
    </ProcessorSubPage>
  );
};

export default SelectLocations;
