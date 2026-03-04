import '../css/animations.css';

export const LoadingIndicator = () => {
  return (
    <div className="absolute inset-0 grid place-items-center">
      <img
        src="/nex-logo-19x.png"
        className="pixelated w-12 loading-bounce-spin-animation"
      />
    </div>
  );
};

export default LoadingIndicator;
