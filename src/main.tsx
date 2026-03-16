import React from 'react';
import ReactDOM from 'react-dom/client';
import NotificationContextProvider from './components/contexts/notification-context';
import App from './app';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <NotificationContextProvider>
      <App />
    </NotificationContextProvider>
  </React.StrictMode>
);
