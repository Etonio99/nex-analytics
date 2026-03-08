import MultiSelect, { MultiSelectItem } from '../../../components/multi-select';
import { useLocations } from '../../../hooks/useLocations';

interface LocationSelectProps {
  value: Record<string, boolean>;
  onChange: (state: Record<string, boolean>) => void;
}

const LocationSelect = (props: LocationSelectProps) => {
  const { data } = useLocations();

  if (!data?.data || data.data.length < 1) {
    throw new Error('Data did not contain any locations!');
  }

  const locationData = data.data[0];

  return (
    <MultiSelect
      title="Select Locations"
      value={props.value}
      onChange={props.onChange}
      items={locationData.locations.map((location) => {
        const addressParts = [];
        if (location.street_address) addressParts.push(location.street_address);
        if (location.street_address_2)
          addressParts.push(location.street_address_2);
        if (location.city) addressParts.push(location.city);
        if (location.state || location.zip_code)
          addressParts.push([location.state, location.zip_code].join(' '));

        const description =
          addressParts.length > 0
            ? addressParts.join(', ')
            : 'No address listed';

        return {
          label: location.name,
          description,
          uniqueKey: location.id,
        } as MultiSelectItem;
      })}
    />
  );
};

export default LocationSelect;
