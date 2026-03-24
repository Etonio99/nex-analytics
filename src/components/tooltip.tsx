interface TooltipProps {
  label: string;
  children: React.ReactNode;
}

const Tooltip = (props: TooltipProps) => {
  return (
    <div className="group relative">
      {props.children}
      <div className="bg-sandstone-500 text-sandstone-50 text-xs shadow shadow-sandstone-950/20 rounded-sm -left-2 top-1/2 -translate-x-4/5 group-hover:-translate-x-full -translate-y-1/2 absolute px-1 py-0.5 pointer-events-none opacity-0 group-hover:opacity-100 group-hover:delay-300 transition-all">
        <p>{props.label}</p>
      </div>
    </div>
  );
};

export default Tooltip;
