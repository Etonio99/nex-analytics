'use client';

import { useProcessor } from '../hooks/useProcessor';

interface HomeProps {
  navigate: (page: string) => void;
}

const Home = (props: HomeProps) => {
  const { setProcessor } = useProcessor(false);

  const click = async () => {
    const response = await setProcessor('appointment_slots');
    console.log(response);
    props.navigate('check-api-key');
  };

  return (
    <div className="max-w-xl m-auto">
      <h1 className="text-4xl font-bold mb-4 text-sandstone-300">
        NexHealth Analytics
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
    </div>
  );
};

export default Home;
