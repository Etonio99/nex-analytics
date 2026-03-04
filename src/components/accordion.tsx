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
        className="text-sandstone-950 px-2 py-1 flex justify-between items-center cursor-pointer"
      >
        <div className="flex gap-2 items-center font-bold">
          {props.icon}
          <p>{props.label}</p>
        </div>
        <div
          className={`text-sandstone-300 transition-transform duration-500 ${open ? 'rotate-180' : 'rotate-0'}`}
        >
          <BiChevronDown />
        </div>
      </div>
      <div
        className={`grid overflow-hidden ${open ? 'grid-rows-[1fr]' : 'grid-rows-[0fr]'} transition-[grid-template-rows] duration-500 pl-6`}
      >
        <div className="min-h-0">
          <div className="bg-sandstone-200 rounded-md">
            <p className="p-2">{props.text}</p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Accordion;
