import { ChangeEventHandler } from 'react';

interface InputProps {
  label: string;
  placeholder: string;
  icon?: React.ReactNode;
  value?: string;
  onChange?: ChangeEventHandler<HTMLInputElement, HTMLInputElement>;
}

const Input = (props: InputProps) => {
  return (
    <div className="w-full">
      <p className="text-sandstone-300">{props.label}</p>
      <div className="p-2 rounded-sm bg-sandstone-100 flex gap-1 outline focus-within:outline-4 outline-sandstone-200 items-center">
        {props.icon}
        <input
          type="text"
          placeholder={props.placeholder}
          autoComplete="off"
          className="w-full outline-none"
          value={props.value ?? ''}
          onChange={props.onChange}
        />
      </div>
    </div>
  );
};

export default Input;
