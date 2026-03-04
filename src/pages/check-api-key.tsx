import Button from '../components/button';
import useApiKey from '../hooks/useKey';

interface CheckApiKeyProps {
  advance: () => Promise<boolean>;
  update: (data: any) => Promise<boolean>;
}

const CheckApiKey = (props: CheckApiKeyProps) => {
  const { setApiKey } = useApiKey();

  const update = () => {
    setApiKey('test');
  };

  return (
    <div className="max-w-xl m-auto">
      <h1 className="text-4xl font-bold mb-4 text-sandstone-300">
        Check API Key
      </h1>
      <Button label="Test Key" style="secondary" onClick={update} />
      <Button label="Continue" style="primary" onClick={props.advance} />
    </div>
  );
};

export default CheckApiKey;
