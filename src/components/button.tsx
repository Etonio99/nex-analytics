const ButtonStyles = {
  primary:
    'bg-sandstone-500 text-sandstone-50 not-disabled:hover:bg-sandstone-400',
  secondary:
    'border-2 border-sandstone-500 text-sandstone-500 not-disabled:hover:border-sandstone-400 not-disabled:hover:text-sandstone-400',
  tertiary: 'text-sandstone-500 not-disabled:hover:text-sandstone-400',
  success: 'bg-green-300 text-green-900 not-disabled:hover:bg-green-200',
  warning: 'bg-amber-300 text-amber-900 not-disabled:hover:bg-amber-200',
  danger: 'bg-red-500 text-sandstone-50 not-disabled:hover:bg-red-400',
};

type ButtonStyle =
  | 'primary'
  | 'secondary'
  | 'tertiary'
  | 'success'
  | 'warning'
  | 'danger';

interface ButtonProps {
  label: string;
  style: ButtonStyle;
  icon?: React.ReactNode;
  disabled?: boolean;
  onClick?: () => void;
}

const Button = (props: ButtonProps) => {
  return (
    <button
      onClick={props.onClick}
      className={`rounded-md px-2 py-1 flex gap-1 items-center justify-center cursor-pointer disabled:cursor-not-allowed disabled:saturate-50 disabled:opacity-50 ${ButtonStyles[props.style]}`}
      disabled={props.disabled}
    >
      {props.icon}
      {props.label}
    </button>
  );
};

export default Button;
