import { createBrowserRouter } from 'react-router-dom';
import Landing from './pages/landing';
import AppRoot from './pages/app/index';

export const router = createBrowserRouter([
  { path: '/', element: <Landing /> },
  { path: '/app', element: <AppRoot /> },
])