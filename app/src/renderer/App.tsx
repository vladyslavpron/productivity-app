import { createTheme, ThemeProvider, Paper, Button, Box } from "@mui/material";
import React, { useEffect, useState } from "react";
import TimeSpentBarChart from "./components/TimeSpentBarChart";
import { RootState } from "./store/store";
import { useSelector, useDispatch } from "react-redux";
import { SessionStats, set } from "./store/currentSessionSlice";

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

  async function fetchCurrentSessionStats() {
    const response = await fetch(
      "http://localhost:8000/api/session/current/statistics"
    );

    const sessionStats = (await response.json()) as SessionStats;

    dispatch(set(sessionStats));
  }

  const currentSession = useSelector(
    (state: RootState) => state.currentSession
  );
  const dispatch = useDispatch();

  const [events, setEvents] = useState<Event[]>([]);

  useEffect(() => {
    fetchEvents();
    fetchCurrentSessionStats();
  }, []);

  // TODO: Layout
  // TODO: refactoring

  //  TODO: guess timeline should be done with divs, since it for some reason doesnt really works with charts.
  // Then there will be way too many divs, it will become laggy for sure. I guess need to use the same approach as before, displaying N biggest entries, and smaller ones with "others" title
  return (
    <ThemeProvider theme={theme}>
      <Paper sx={{ width: "100vh", height: "100vh" }}>
        <Box>Total app switches today: {events.length}</Box>
        <Box>
          Total applications used today: {currentSession.time_spent.length}
        </Box>
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
