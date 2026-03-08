import MultiSelect, { MultiSelectItem } from '../../../components/multi-select';
import { useLocations } from '../../../hooks/useLocations';

const LocationSelect = () => {
  const { data } = useLocations();

  console.log(data);

  if (!data?.data || data.data.length < 1) {
    throw new Error('Data did not contain any locations!');
  }

  const locationData = data.data[0];

  return (
    <MultiSelect
      items={locationData.locations.map((location) => {
        const description = `${location.street_address}${location.street_address_2 ? `, ${location.street_address_2}` : ''}, ${location.city}, ${location.state} ${location.zip_code}`;

        return {
          label: location.name ?? '(Unnamed Location)',
          description,
          uniqueKey: location.id,
        } as MultiSelectItem;
      })}
    />
  );
};

export default LocationSelect;
