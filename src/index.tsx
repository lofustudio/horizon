import "./index.css";

import React from "react";
import ReactDOM from "react-dom/client";
import {
  RouterProvider,
  Router,
  RootRoute,
  Route,
} from '@tanstack/react-router';
import Index from "./routes";
import { ThemeProvider } from "./context/theme";

const rootRoute = new RootRoute();

const indexRoute = new Route({
  getParentRoute: () => rootRoute,
  path: "/",
  component: Index,
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
        <RouterProvider router={router} />
      </ThemeProvider>
    </React.StrictMode>,
  )
}