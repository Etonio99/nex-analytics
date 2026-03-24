interface NotFoundProps {
  navigate: (page: string) => void;
}

const NotFound = (props: NotFoundProps) => {
  return (
    <div className="max-w-xl m-auto space-y-1 h-full flex flex-col justify-center items-center text-center">
      <h1 className="text-6xl font-bold mb-4 text-sandstone-300">404</h1>
      <p className="w-4/5">
        Uh oh! This page doesn't seem to exist. Please{' '}
        <span
          className="text-teal-500 cursor-pointer"
          onClick={() => props.navigate('home')}
        >
          click here to go back
        </span>{' '}
        to the home page.
      </p>
    </div>
  );
};

export default NotFound;
