import ReactDOM from "react-dom/client";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import Page from "./components/layouts/page";
import Items from "./components/layouts/items";
import Item, { pricesLoader } from "./components/routes/item";
import "./styles/index.css";
import {
  CssBaseline,
  ThemeProvider,
  Typography,
  createTheme,
} from "@mui/material";
import Add from "./components/routes/add";
import React from "react";

const darkTheme = createTheme({
  palette: {
    mode: "dark",
  },
});

const router = createBrowserRouter([
  {
    path: "/",
    element: <Page />,
    children: [
      { path: "items/", element: <Items /> },
      { path: "items/add", element: <Add /> },
      {
        path: "items/:id",
        loader: pricesLoader,
        errorElement: (
          <Typography variant="h3">Что-то пошло не так!</Typography>
        ),
        element: (
          <>
            <Items />
            <Item />
          </>
        ),
      },
    ],
  },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      <RouterProvider router={router} />
    </ThemeProvider>
  </React.StrictMode>
);
