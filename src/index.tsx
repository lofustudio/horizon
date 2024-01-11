import "./index.css";

import React from "react";
import ReactDOM from "react-dom/client";
import {
  RouterProvider,
  Router,
  RootRoute,
  Route,
} from '@tanstack/react-router';

// Contexts
import { ThemeProvider } from "./context/theme";

// Components
import WindowBar from "./components/app/window-bar";

// Routes
import YourLibraryPage from "./routes/library";
import LibraryBar from "./components/app/library-bar";

const rootRoute = new RootRoute();

const indexRoute = new Route({
  getParentRoute: () => rootRoute,
  path: "/",
  component: YourLibraryPage,
});

const routeTree = rootRoute.addChildren([indexRoute])

const router = new Router({ routeTree })

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}

const rootElement = document.getElementById('root') as HTMLElement;

if (!rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement)
  root.render(
    <React.StrictMode>
      <ThemeProvider defaultTheme="dark" storageKey="theme">
        <main className="flex flex-col w-full min-h-screen">
          <nav id='nav' className='fixed z-[2000] top-0 w-full h-[36px]'>
            <WindowBar />
          </nav>
          <div className="flex flex-row w-full">
            <LibraryBar />
            <RouterProvider router={router} />
          </div>
        </main>
      </ThemeProvider>
    </React.StrictMode>,
  )
}