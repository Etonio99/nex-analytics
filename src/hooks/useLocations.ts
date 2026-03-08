import { useSuspenseQuery } from '@tanstack/react-query';
import { useController } from './useController';

const useLocations = () => {
  const { getLocations } = useController();

  return useSuspenseQuery({
    queryKey: ['locations'],
    queryFn: () => getLocations(),
  });
};

export { useLocations };
