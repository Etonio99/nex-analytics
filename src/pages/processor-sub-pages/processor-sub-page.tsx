interface ProcessorSubPageProps {
  children: React.ReactNode;
  title: string;
}

const ProcessorSubPage = (props: ProcessorSubPageProps) => {
  return (
    <div className="max-w-xl m-auto">
      <h1 className="text-4xl font-bold mb-4 text-sandstone-300">
        {props.title}
      </h1>
      <div>{props.children}</div>
    </div>
  );
};

export default ProcessorSubPage;
