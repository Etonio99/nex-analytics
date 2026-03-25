'use client';

import { BiLogoGithub, BiSolidCog, BiSolidHelpCircle } from 'react-icons/bi';
import { useProcessor } from '../hooks/useProcessor';
import Tooltip from '../components/tooltip';

interface HomeProps {
  navigate: (page: string) => void;
}

const Home = (props: HomeProps) => {
  const { setProcessor } = useProcessor();

  const click = async () => {
    const response = await setProcessor('appointment_slots');
    console.log(response);
    props.navigate('process');
  };

  return (
    <div className="max-w-xl m-auto">
      <h1 className="text-4xl font-bold mb-4 text-sandstone-300">
        Nex Analytics
      </h1>

      <button
        onClick={click}
        className="text-left shadow shadow-sandstone-900/20 rounded-md border border-sandstone-300 px-4 pt-2 pb-3 hover:bg-sandstone-100 hover:-translate-y-1 transition-transform cursor-pointer"
      >
        <h3 className="font-bold text-sandstone-400 text-xl">
          Appointment Slots Report
        </h3>
        <p>
          Export appointment slots for any number of locations within the next X
          days.
        </p>
      </button>
      <div className="flex flex-col gap-1 absolute bottom-4 right-4">
        <Tooltip label="GitHub">
          <a
            href="https://github.com/Etonio99/nex-analytics"
            target="_blank"
            className="group p-2 rounded-sm text-sandstone-500 hover:bg-sandstone-100 grid place-items-center"
          >
            <BiLogoGithub
              size={28}
              className="group-hover:rotate-12 transition-transform group-hover:text-sandstone-600"
            />
          </a>
        </Tooltip>
        <Tooltip label="Help">
          <button
            onClick={() => props.navigate('help')}
            className="group p-2 rounded-sm text-sandstone-500 hover:bg-sandstone-100 grid place-items-center"
          >
            <BiSolidHelpCircle
              size={28}
              className="group-hover:rotate-12 transition-transform group-hover:text-sandstone-600"
            />
          </button>
        </Tooltip>
      </div>
    </div>
  );
};

export default Home;
