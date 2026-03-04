import { useEffect, useState } from 'react';

export type MultiSelectItem = {
  label: string;
  description: string;
  uniqueKey: string;
  checked?: boolean;
};

interface MultiSelectProps {
  items: MultiSelectItem[];
  onChange?: (state: Record<string, boolean>) => void;
}

const MultiSelect = (props: MultiSelectProps) => {
  const [keyState, setKeyState] = useState<Record<string, boolean>>({});

  useEffect(() => {
    const initializeKeyState = () => {
      const newState: Record<string, boolean> = {};
      for (const item of props.items) {
        newState[item.uniqueKey] = item.checked ?? false;
      }
      setKeyState(newState);
      props.onChange?.(newState);
    };

    initializeKeyState();
  }, []);

  const handleChange = (key: string) => {
    if (!key) {
      console.error('handleChange called with invalid key:', key);
      return;
    }

    setKeyState((previous) => ({
      ...previous,
      [key]: !previous[key],
    }));

    console.log(keyState);
  };

  return (
    <div>
      <div className="px-2 py-1 bg-sandstone-50">
        <p className="text-lg font-bold">
          Select which locations you want to query
        </p>
        <p className="text-sandstone-400 text-sm">
          This may save you on API calls
        </p>
      </div>
      <div className="overflow-y-auto max-h-64">
        <ul className="p-2 space-y-2 bg-sandstone-200 rounded-md">
          {props.items.map((item) => (
            <MultiSelectItem
              key={item.uniqueKey}
              uniqueKey={item.uniqueKey}
              label={item.label}
              description={item.description}
              checked={keyState[item.uniqueKey] ?? false}
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
  uniqueKey: string;
  checked: boolean;
  onChange: (key: string) => void;
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
        <p className="relative top-0.5">{props.label}</p>
        <p className="text-sandstone-300 relative bottom-0.5">
          {props.description}
        </p>
      </div>
    </li>
  );
};

export default MultiSelect;
