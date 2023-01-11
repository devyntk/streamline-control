import { createBrowserRouter, RouterProvider } from "react-router-dom";
import React from "react";
import { createRoot } from "react-dom/client";

import "./styles.css";
import "rsuite/dist/rsuite.min.css";
import Dashboard from "./pages/Dashboard";
import Login from "./pages/Login";
import { CustomProvider } from "rsuite";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Dashboard />,
  },
  {
    path: "/login",
    element: <Login />,
  },
]);

const container = document.getElementById("app");
const root = createRoot(container!);
root.render(
  <CustomProvider>
    <React.StrictMode>
      <RouterProvider router={router} />
    </React.StrictMode>
  </CustomProvider>
);
