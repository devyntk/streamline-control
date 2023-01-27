import { createBrowserRouter, RouterProvider } from "react-router-dom";
import React from "react";
import { createRoot } from "react-dom/client";

import "./styles.less";
import "rsuite/styles/index.less";
import Dashboard from "~/pages/Dashboard";
import Login from "~/pages/Login";
import { CustomProvider } from "rsuite";
import Home from "~/pages/Home";
import Status from "~/pages/Status";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Dashboard />,
    children: [
      {
        index: true,
        element: <Home />,
      },
      {
        path: "status",
        element: <Status />,
      },
    ],
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
