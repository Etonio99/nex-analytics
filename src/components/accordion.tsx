'use client';

import { useState } from 'react';
import { BiChevronDown } from 'react-icons/bi';

interface AccordionProps {
  icon: React.ReactNode;
  label: string;
  text: string;
}

const Accordion = (props: AccordionProps) => {
  const [open, setOpen] = useState(false);

  return (
    <div className="rounded-md overflow-hidden">
      <div
        onClick={() => setOpen(!open)}
        className="text-sandstone-800 flex justify-between cursor-pointer mb-1"
      >
        <div className="grid place-items-center h-fit py-2">{props.icon}</div>
        <span className="flex w-full h-full items-center hover:bg-sandstone-100 rounded-sm pr-2 py-1 ml-2">
          <p className="font-bold w-full ml-2 text-left">{props.label}</p>
          <div
            className={`text-sandstone-300 transition-transform duration-500 ${open ? 'rotate-180' : 'rotate-0'}`}
          >
            <BiChevronDown />
          </div>
        </span>
      </div>
      <div
        className={`grid overflow-hidden ${open ? 'grid-rows-[1fr]' : 'grid-rows-[0fr]'} transition-[grid-template-rows] duration-500 pl-6`}
      >
        <div className="min-h-0">
          <div className="bg-sandstone-200 rounded-md">
            <p className="p-2 text-sm">{props.text}</p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Accordion;
