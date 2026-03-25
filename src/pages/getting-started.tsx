import { BiArrowBack } from 'react-icons/bi';
import MarkdownFile from '../components/markdown-file';

interface GettingStartedProps {
  navigate: (page: string) => void;
}

const GettingStarted = (props: GettingStartedProps) => {
  return (
    <div className="max-w-xl m-auto space-y-1 h-full flex flex-col">
      <h1 className="text-4xl font-bold mb-4 text-sandstone-300">
        Getting Started
      </h1>

      <MarkdownFile filePath="getting-started.md" />

      <button
        onClick={() => props.navigate('help')}
        className="flex items-center gap-2 absolute left-4 top-4 px-2 py-1 rounded-sm hover:bg-sandstone-100 text-sandstone-500"
      >
        <BiArrowBack /> Back
      </button>
    </div>
  );
};

export default GettingStarted;
