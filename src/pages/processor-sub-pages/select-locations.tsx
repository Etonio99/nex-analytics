import { useEffect, useState } from 'react';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import ProcessorSubPage from './processor-sub-page';
import Button from '../../components/button';
import MultiSelect, { MultiSelectItem } from '../../components/multi-select';

const SelectLocations = (props: ProcessSubPageProps) => {
  const getInitialSelectedLocations = (): number[] => {
    if (props.advanceResult?.error?.type === 'LOCATION_REQUIRED') {
      if (props.advanceResult.error.resolutionData?.type === 'LOCATIONS') {
        return (
          props.advanceResult.error.resolutionData.payload
            .selected_location_ids ?? []
        );
      }
    }
    return [];
  };

  const [locationSelection, setLocationSelection] = useState<
    Record<number, boolean>
  >({});

  const continueProcess = async () => {
    const selected_location_ids = Object.entries(locationSelection)
      .filter(([_, selected]) => selected)
      .map(([id, _]) => parseInt(id));
    await props.appActions.updateProcessorData({ selected_location_ids });
    await props.appActions.advanceProcessor();
  };

  const locations =
    props.advanceResult?.error?.resolutionData?.type === 'LOCATIONS'
      ? props.advanceResult.error.resolutionData.payload.locations
      : [];

  const selectedCount = Object.values(locationSelection).filter(Boolean).length;

  useEffect(() => {
    const entries: Record<number, boolean> = {};
    const initialSelectedLocations = getInitialSelectedLocations();
    locations.forEach((location) => {
      entries[location.id] = initialSelectedLocations.includes(location.id);
    });
    setLocationSelection(entries);
  }, []);

  return (
    <ProcessorSubPage title="Select Locations">
      <MultiSelect
        title="Select locations"
        description="Choose which locations you want to collect data from"
        value={locationSelection}
        onChange={setLocationSelection}
        items={locations.map((location) => {
          const addressParts = [
            location.street_address,
            location.street_address_2,
            location.city,
            location.state
              ? `${location.state} ${location.zip_code}`
              : location.zip_code,
          ].filter(Boolean);

          const description = addressParts.join(', ') || 'No address listed';

          return {
            label: location.name,
            description,
            uniqueKey: location.id,
          } as MultiSelectItem;
        })}
      />
      <div className="flex justify-end mt-2">
        <Button
          label="Save"
          style="primary"
          onClick={continueProcess}
          disabled={selectedCount < 1}
        />
      </div>
    </ProcessorSubPage>
  );
};

export default SelectLocations;
