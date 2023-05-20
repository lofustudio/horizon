import { RouterProvider, createBrowserRouter } from 'react-router-dom'
import ReactDOM from 'react-dom/client'
import React from 'react'

import App from './app.tsx'

import Index from './pages/index/page.tsx'
import IndexError from './pages/index/error.tsx'

import './index.css'

const router = createBrowserRouter([
    {
        path: "/",
        element: <Index />,
        errorElement: <IndexError />
    }
]);

ReactDOM.createRoot(document.getElementById('app') as HTMLElement).render(
    <React.StrictMode>
        <App>
            <RouterProvider router={router} />
        </App>
    </React.StrictMode>,
)
