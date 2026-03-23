interface ProcessorSubPageProps {
  children: React.ReactNode;
  title: string;
  description?: string;
}

const ProcessorSubPage = (props: ProcessorSubPageProps) => {
  return (
    <div className="w-full">
      <h1 className="text-4xl font-bold mb-4 text-sandstone-300">
        {props.title}
      </h1>
      {props.description && <p className="mb-3">{props.description}</p>}
      <div>{props.children}</div>
    </div>
  );
};

export default ProcessorSubPage;
