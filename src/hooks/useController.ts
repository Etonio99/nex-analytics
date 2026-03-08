import { invoke } from '@tauri-apps/api/core';
import { NexApiResponse } from '../types/api/nex-api';
import { InstitutionLocations } from '../types/api/locations';
import { useCallback } from 'react';

export const useController = () => {
  const getLocations = useCallback(
    async (
      inactive?: boolean
    ): Promise<NexApiResponse<InstitutionLocations[]>> =>
      invoke('get_locations', {
        inactive: inactive ?? false,
      }),
    []
  );

  return {
    getLocations,
  };
};
