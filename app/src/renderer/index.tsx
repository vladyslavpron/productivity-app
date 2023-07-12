import React from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import { Provider } from "react-redux";
import { store } from "./store/store";
import {
  StyledEngineProvider,
  ThemeProvider,
  createTheme,
} from "@mui/material";

const container = document.getElementById("app-root")!;

const root = createRoot(container);

const theme = createTheme({
  palette: {
    primary: {
      main: "#F5D8D6",
      dark: "#32320C",
      light: "#F3F33F",
    },
  },
});

root.render(
  <StyledEngineProvider injectFirst>
    <ThemeProvider theme={theme}>
      <Provider store={store}>
        <App />
      </Provider>
    </ThemeProvider>
  </StyledEngineProvider>
);
