import { createBrowserRouter } from 'react-router';

import Register from '@/pages/auth/Register';
import Home from '../pages/auth/Home';
import Login from '../pages/auth/Login';

export const router = createBrowserRouter([
  {
    path: '/',
    element: <Login />,
  },
  {
    path: '/home',
    element: <Home />,
  },
  {
    path: '/Register',
    element: <Register />,
  },
]);
