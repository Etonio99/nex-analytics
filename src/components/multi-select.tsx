export type MultiSelectItem = {
  label: string;
  description: string;
  uniqueKey: string | number;
  checked?: boolean;
};

interface MultiSelectProps {
  title: string;
  description?: string;
  items: MultiSelectItem[];
  value: Record<string, boolean>;
  onChange: (state: Record<string, boolean>) => void;
}

const MultiSelect = (props: MultiSelectProps) => {
  const handleChange = (key: string | number) => {
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

  return (
    <div>
      <div className="px-2 py-1 bg-sandstone-50">
        <p className="text-lg font-bold">{props.title}</p>
        {props.description && (
          <p className="text-sandstone-400 text-sm">{props.description}</p>
        )}
      </div>
      <div className="overflow-y-auto max-h-64">
        <ul className="p-2 space-y-2 bg-sandstone-200 rounded-md">
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
    </div>
  );
};

interface MultiSelectItemProps {
  label: string;
  description: string;
  uniqueKey: string | number;
  checked: boolean;
  onChange: (key: string | number) => void;
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
