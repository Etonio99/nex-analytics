import LoadingIndicator from '../../components/loading-indicator';

type LoadingProps = {
  message?: string;
};

const Loading = ({ message = '' }: LoadingProps) => {
  return (
    <div className="grid place-items-center absolute inset-0">
      <LoadingIndicator />
      <p className="top-13 relative text-sm text-sandstone-500">{message}</p>
    </div>
  );
};

export default Loading;
