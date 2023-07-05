import { createTheme, ThemeProvider, Paper, Button } from "@mui/material";
import React, { useEffect, useState } from "react";
import {
  LineChart,
  CartesianGrid,
  XAxis,
  YAxis,
  Tooltip,
  Line,
  Legend,
  ResponsiveContainer,
} from "recharts";
import TimeSpentBarChart from "./TimeSpentBarChart";

const theme = createTheme({
  palette: {
    primary: {
      main: "#F5D8D6",
      dark: "#32320C",
      light: "#F3F33F",
    },
  },
});

function App() {
  async function fetchEvents() {
    const response = await fetch("http://localhost:8000/api/event");
    const events = (await response.json()) as Event[];

    setEvents(events);
  }

  const [events, setEvents] = useState<Event[]>([]);

  const titles = events.map((event) => event.title);

  useEffect(() => {
    fetchEvents();
  }, []);
  // TODO: Current session charts, make API endpoint with stats for session

  // TODO: Store
  // TODO: Layout
  // TODO: refactoring

  //  TODO: guess timeline should be done with divs, since it for some reason doesnt really works with charts
  return (
    <ThemeProvider theme={theme}>
      <Paper sx={{ width: "100vh", height: "100vh" }}>
        <Button variant="contained">Contained</Button>
        Hello wrodl11231!
        <TimeSpentBarChart />
      </Paper>
    </ThemeProvider>
  );
}

export default App;

interface Event {
  id: number;
  path: string;
  title: string;
  timestamp: string;
  offset: number;
  session_id: number;
}
