'use client';

import { useProcessor } from '../hooks/useProcessor';

const CheckApiKey = () => {
  useProcessor(true);

  return (
    <div className="max-w-xl m-auto">
      <h1 className="text-4xl font-bold mb-4 text-sandstone-300">
        Check API Key
      </h1>
    </div>
  );
};

export default CheckApiKey;
