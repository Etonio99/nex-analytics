'use client';

import { createContext, useContext, useMemo, useState } from 'react';
import { createApiClient } from '../../utils/api/client';

interface AppContextValue {
  apiToken: string | null;
  setApiToken: React.Dispatch<React.SetStateAction<string | null>>;
}

const AppContext = createContext<AppContextValue | null>(null);

interface AppContextProviderProps {
  children: React.ReactNode;
}

export default function AppContextProvider(props: AppContextProviderProps) {
  const [apiToken, setApiToken] = useState<string | null>(null);
  const [subdomain, setSubdomain] = useState<string | null>(null);

  const apiClient = useMemo(() => createApiClient(() => apiToken), [apiToken]);

  return (
    <AppContext.Provider
      value={{
        apiToken,
        setApiToken,
      }}
    >
      {props.children}
    </AppContext.Provider>
  );
}

export const useAppContext = () => {
  const context = useContext(AppContext);
  if (!context) {
    throw new Error('useAppContext must be used within a AppContextProvider');
  }
  return context;
};
