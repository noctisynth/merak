import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { createBrowserRouter } from 'react-router';
import { RouterProvider } from 'react-router/dom';
import './index.css';

import Login from './pages/login';
import Register from './pages/register';

function Home() {
  return (
    <div className="min-h-screen bg-background p-6 text-foreground">
      <h1 className="text-2xl font-semibold">Home</h1>
      <p className="mt-2 text-muted-foreground">You are logged in.</p>
    </div>
  );
}

const router = createBrowserRouter([
  {
    path: '/login',
    element: <Login />,
  },
  {
    path: '/home',
    element: <Home />,
  },
  {
    path: '/register',
    element: <Register />,
  },
]);

const root = document.getElementById('root') as HTMLDivElement;

createRoot(root).render(
  <StrictMode>
    <RouterProvider router={router} />
  </StrictMode>,
);
