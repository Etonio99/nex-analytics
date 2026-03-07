import { Suspense } from 'react';
import MultiSelect from '../../components/multi-select';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import ProcessorSubPage from './processor-sub-page';

const SelectLocations = (props: ProcessSubPageProps) => {
  return (
    <ProcessorSubPage title="Select Locations">
      <Suspense>
        <MultiSelect />
      </Suspense>
    </ProcessorSubPage>
  );
};

export default SelectLocations;
