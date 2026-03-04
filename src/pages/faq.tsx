import {
  BiAbacus,
  BiMoneyWithdraw,
  BiSolidKey,
  BiSolidUser,
} from 'react-icons/bi';
import Accordion from '../components/accordion';

const FAQ = () => {
  return (
    <div className="max-w-xl m-auto">
      <Accordion
        icon={<BiSolidKey />}
        label="What is an API key?"
        text="IDK, ask me later"
      />
      <Accordion
        icon={<BiAbacus />}
        label="How many API calls will this make?"
        text="IDK, ask me later"
      />
      <Accordion
        icon={<BiMoneyWithdraw />}
        label="How much does this cost?"
        text="IDK, ask me later"
      />
      <Accordion
        icon={<BiSolidUser />}
        label="Who made this?"
        text="IDK, ask me later"
      />
    </div>
  );
};

export default FAQ;
