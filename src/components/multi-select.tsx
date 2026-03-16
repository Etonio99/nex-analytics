export type MultiSelectItem = {
  label: string;
  description: string;
  uniqueKey: number;
  checked?: boolean;
};

interface MultiSelectProps {
  title: string;
  description?: string;
  items: MultiSelectItem[];
  value: Record<number, boolean>;
  onChange: (state: Record<number, boolean>) => void;
}

const MultiSelect = (props: MultiSelectProps) => {
  const handleChange = (key: number) => {
    if (!key) {
      console.error('handleChange called with invalid key:', key);
      return;
    }

    const newState = {
      ...props.value,
      [key]: !props.value[key],
    };
    props.onChange(newState);
  };

  const setAll = (setting: boolean) => {
    const newState = Object.fromEntries(
      Object.keys(props.value).map((k) => [k, setting])
    );
    props.onChange(newState);
  };

  const areAllSelected = (): boolean =>
    Object.values(props.value).every((v) => v === true);

  const selectedCount = (): number =>
    Object.values(props.value).filter(Boolean).length;

  return (
    <div>
      <div className="px-2 py-1 bg-sandstone-50">
        <p className="text-lg font-bold">{props.title}</p>
        <div className="flex justify-between text-sm">
          {props.description && (
            <p className="text-sandstone-400">{props.description}</p>
          )}
          {areAllSelected() && (
            <button
              className="text-teal-500 text-xs"
              onClick={() => setAll(false)}
            >
              Deselect All
            </button>
          )}
          {!areAllSelected() && (
            <button
              className="text-teal-500 text-xs"
              onClick={() => setAll(true)}
            >
              Select All
            </button>
          )}
        </div>
      </div>
      <div className="overflow-y-auto max-h-64 rounded-md overflow-hidden bg-sandstone-200">
        <ul className="p-2 space-y-2">
          {props.items.map((item) => (
            <MultiSelectItem
              key={item.uniqueKey}
              uniqueKey={item.uniqueKey}
              label={item.label}
              description={item.description}
              checked={!!props.value[item.uniqueKey]}
              onChange={handleChange}
            />
          ))}
        </ul>
      </div>
      <div className="flex justify-end mt-2">
        <p className="text-xs text-sandstone-300">{`${selectedCount()} of ${Object.keys(props.value).length} selected`}</p>
      </div>
    </div>
  );
};

interface MultiSelectItemProps {
  label: string;
  description: string;
  uniqueKey: number;
  checked: boolean;
  onChange: (key: number) => void;
}

const MultiSelectItem = (props: MultiSelectItemProps) => {
  const toggle = () => {
    props.onChange(props.uniqueKey);
  };

  return (
    <li
      onClick={toggle}
      className={`${props.checked ? '' : 'brightness-80'} bg-sandstone-50 grid grid-cols-[40px_1fr] rounded-sm overflow-hidden shadow shadow-sandstone-300 cursor-pointer`}
    >
      <div className="grid place-items-center bg-sandstone-100">
        <input
          type="checkbox"
          checked={props.checked}
          className="pointer-events-none"
          readOnly
        />
      </div>
      <div className="pl-2">
        {props.label !== '' && (
          <p className="relative top-0.5">{props.label}</p>
        )}
        {(props.label === '' || !props.label) && (
          <p className="relative top-0.5 opacity-75 italic">Unnamed</p>
        )}
        <p className="text-sandstone-300 relative bottom-0.5">
          {props.description}
        </p>
      </div>
    </li>
  );
};

export default MultiSelect;
