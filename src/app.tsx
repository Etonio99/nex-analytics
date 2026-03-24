'use client';

import Home from './pages/home';
import FAQ from './pages/faq';
import Process from './pages/process';
import './css/app.css';
import { useState } from 'react';
import Help from './pages/help';
import GettingStarted from './pages/getting-started';
import NotFound from './pages/not-found';
const App = () => {
  const [page, setPage] = useState('home');

  const getPage = (pageName: string) => {
    switch (pageName) {
      case 'home':
        return <Home navigate={setPage} />;
      case 'help':
        return <Help navigate={setPage} />;
      case 'getting-started':
        return <GettingStarted navigate={setPage} />;
      case 'faq':
        return <FAQ navigate={setPage} />;
      case 'process':
        return <Process navigate={setPage} />;
    }
    return <NotFound navigate={setPage} />;
  };

  return (
    <main className="h-screen p-4 overflow-hidden w-screen">
      {getPage(page)}
    </main>
  );
};

export default App;
