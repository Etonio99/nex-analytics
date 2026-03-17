'use client';

import Home from './pages/home';
import FAQ from './pages/faq';
import Process from './pages/process';

import './css/app.css';
import { useState } from 'react';
const App = () => {
  const [page, setPage] = useState('home');

  const getPage = (pageName: string) => {
    switch (pageName) {
      case 'home':
        return <Home navigate={setPage} />;
      case 'faq':
        return <FAQ />;
      case 'process':
        return <Process />;
    }
    return null;
  };

  return (
    <main className="h-screen p-4 overflow-hidden w-screen">
      {getPage(page)}
    </main>
  );
};

export default App;
