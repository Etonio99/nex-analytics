import { invoke } from '@tauri-apps/api/core';

const useApiKey = () => {
  const setApiKey = async (key: string) =>
    invoke('save_api_key', { key })
      .then(() => true)
      .catch(() => false);

  const getApiKey = async () =>
    invoke('get_api_key')
      .then(() => true)
      .catch(() => false);

  return { setApiKey, getApiKey };
};

export default useApiKey;
