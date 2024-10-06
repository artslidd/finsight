import { createRoot } from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";

import App from "./App";
import AddAsset from "./AddAsset";

const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
  },
  {
    path: "add-asset",
    element: <AddAsset />,
  },
]);

createRoot(document.getElementById("root") as HTMLElement).render(
  <RouterProvider router={router} />,
);
