import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { createBrowserRouter, RouterProvider } from 'react-router';
import './i18n';
import './index.css';

import AppRoot from './pages/app/index';
import Landing from './pages/landing';

const router = createBrowserRouter([
  { path: '/', element: <Landing /> },
  { path: '/app/*', element: <AppRoot /> },
]);

const root = document.getElementById('root') as HTMLDivElement;

createRoot(root).render(
  <StrictMode>
    <RouterProvider router={router} />
  </StrictMode>,
);
