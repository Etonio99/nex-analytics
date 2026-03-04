import { useState, useEffect } from 'react';

export const useRouter = () => {
  const [page, setPage] = useState(
    window.location.hash.replace('#', '') || 'home'
  );

  useEffect(() => {
    const handleHashChange = () => {
      setPage(window.location.hash.replace('#', '') || 'home');
    };

    window.addEventListener('hashchange', handleHashChange);
    return () => window.removeEventListener('hashchange', handleHashChange);
  }, []);

  const navigate = (newPage: string) => {
    window.location.hash = newPage;
  };

  return { page, navigate };
};
