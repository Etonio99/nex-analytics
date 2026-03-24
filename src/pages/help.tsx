import {
  BiArrowBack,
  BiListCheck,
  BiRightTopArrowCircle,
} from 'react-icons/bi';

interface HelpProps {
  navigate: (page: string) => void;
}

const Help = (props: HelpProps) => {
  return (
    <div className="max-w-xl m-auto space-y-1 h-full flex flex-col">
      <h1 className="text-4xl font-bold mb-4 text-sandstone-300">Help</h1>
      <p>
        Welcome to the Nex Analytics tool! Please view the links below to
        receive support on taking actions within this application.
      </p>

      <div className="grid grid-cols-3 place-items-center mt-3">
        <button
          onClick={() => props.navigate('getting-started')}
          className="flex flex-col w-42 h-18 gap-1 justify-center items-center rounded-sm outline outline-sandstone-300 px-3 py-2 hover:bg-sandstone-100 hover:-translate-y-1 shadow shadow-sandstone-900/20 transition-transform"
        >
          <BiRightTopArrowCircle size={28} className="text-sandstone-400" />
          <p className="text-sm">Getting Started Guide</p>
        </button>
        <button
          onClick={() => props.navigate('faq')}
          className="flex flex-col w-42 h-18 gap-1 justify-center items-center rounded-sm outline outline-sandstone-300 px-3 py-2 hover:bg-sandstone-100 hover:-translate-y-1 shadow shadow-sandstone-900/20 transition-transform"
        >
          <BiListCheck size={28} className="text-sandstone-400" />
          <p className="text-sm">Visit the FAQ</p>
        </button>
        <button
          onClick={() => props.navigate('not-found')}
          className="flex flex-col w-42 h-18 gap-1 justify-center items-center rounded-sm outline outline-sandstone-300 px-3 py-2 hover:bg-sandstone-100 hover:-translate-y-1 shadow shadow-sandstone-900/20 transition-transform"
        >
          <BiListCheck size={28} className="text-sandstone-400" />
          <p className="text-sm">Test</p>
        </button>
      </div>

      <button
        onClick={() => props.navigate('home')}
        className="flex items-center gap-2 absolute left-4 top-4 px-2 py-1 rounded-sm hover:bg-sandstone-100 text-sandstone-500"
      >
        <BiArrowBack /> Back
      </button>
    </div>
  );
};

export default Help;
