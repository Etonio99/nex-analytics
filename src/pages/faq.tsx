import {
  BiAbacus,
  BiArrowBack,
  BiBuildings,
  BiMoneyWithdraw,
  BiSolidBarChartSquare,
  BiSolidHelpCircle,
  BiSolidKey,
  BiSolidLock,
  BiSolidUser,
} from 'react-icons/bi';
import Accordion from '../components/accordion';

interface FAQProps {
  navigate: (page: string) => void;
}

const FAQ = (props: FAQProps) => {
  return (
    <div className="max-w-xl m-auto space-y-1 h-full flex flex-col">
      <h1 className="text-4xl font-bold mb-4 text-sandstone-300">FAQ</h1>
      <p>
        This page contains commonly asked questions about the Nex Analytics
        tool, the NexHealth API, and more.
      </p>
      <div className="overflow-y-scroll flex-1 min-h-0 space-y-1 border-y border-sandstone-200 mt-3 pb-3 pr-2">
        <h2 className="text-sandstone-500 mt-3">API Keys</h2>
        <Accordion
          icon={<BiSolidHelpCircle />}
          label="What is an API key?"
          text="An API key is a code used to authenticate requests made through the NexHealth API. It ties the request back to the user who made the request. An API key is required to use this tool."
        />
        <Accordion
          icon={<BiSolidKey />}
          label="How do I get an API key?"
          text="You can sign up for the NexHealth API at https://developers.nexhealth.com/signup. An API key will be provided after signing up."
        />
        <Accordion
          icon={<BiBuildings />}
          label="What is a subdomain?"
          text='Subdomains are formatted versions of practice names that are used when accessing data through the API. They typically are the name of the practice with all lowercase letters and dashes instead of spaces (for example, "My Dental Office" will become "my-dental-office").'
        />
        <Accordion
          icon={<BiSolidLock />}
          label="It says I don't have access to my subdomain. How do I gain access?"
          text="By default, API keys will not allow you to access any practice. Instead, authorization will need to be granted per location. To gain access to a location, NexHealth will need an email to developers@nexhealth.com from an adminsitrator at the practice authorizing the access. Please include the address of the location as well."
        />

        <h2 className="text-sandstone-500 mt-6">Understanding the Data</h2>
        <Accordion
          icon={<BiSolidBarChartSquare />}
          label="Why is my report different than what I see in my EHR?"
          text="IDK, ask me later"
        />

        <h2 className="text-sandstone-500 mt-6">Pricing</h2>
        <Accordion
          icon={<BiAbacus />}
          label="How many API calls will this make?"
          text="This can depend on the analytics you are collecting. An API call will be made to get a list of locations within a subdomain, to get available slots per location, etc. The exported reports will tell you the number of API calls that were made while collecting the data."
        />
        <Accordion
          icon={<BiMoneyWithdraw />}
          label="How much does this cost?"
          text="By default, the API costs $0.10 per API call. The current pricing can be found at https://synchronizer.io/pricing."
        />
        <Accordion
          icon={<BiSolidUser />}
          label="Who made this?"
          text="IDK, ask me later"
        />
      </div>

      <button
        onClick={() => props.navigate('help')}
        className="flex items-center gap-2 absolute left-4 top-4 px-2 py-1 rounded-sm hover:bg-sandstone-100 text-sandstone-500"
      >
        <BiArrowBack /> Back
      </button>
    </div>
  );
};

export default FAQ;
