import {
  createTheme,
  ThemeProvider,
  Paper,
  Button,
  Box,
  Typography,
} from "@mui/material";
import React, { useEffect, useState } from "react";
import TimeSpentBarChart from "./components/TimeSpentBarChart";
import { RootState } from "./store/store";
import { useSelector, useDispatch } from "react-redux";
import { SessionStats, set } from "./store/currentSessionSlice";
import Navbar from "./components/Navbar";
import Timeline from "./components/Timeline";

function App() {
  async function fetchEvents() {
    const response = await fetch(
      "http://localhost:8000/api/session/current/events"
    );
    const events = (await response.json()) as Event[];
    setEvents(events);
  }

  async function fetchCurrentSessionStats() {
    const response = await fetch(
      "http://localhost:8000/api/session/current/statistics"
    );

    const sessionStats = (await response.json()) as SessionStats;
    console.log(sessionStats);

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

  // TODO: list/table of events with all events, but implement virtual scroll for performance since sometimes there are a lot of em

  const currentApplication =
    currentSession.appVisitedEntries[
      currentSession.appVisitedEntries.length - 1
    ]?.app_title;
  return (
    <Paper className="md:container md:mx-auto">
      <Navbar />
      {currentSession.totalTimeInApps && (
        <>
          <Box>
            <Typography>Total app switches today: {events.length}</Typography>
            <Typography>
              Total applications used today: {currentSession.timePerApp.length}
            </Typography>
            <Typography>
              Average consecutive time in application:
              {currentSession.avgTimeInApp}
            </Typography>
            <Typography>
              Total time in application: {currentSession.totalTimeInApps}
            </Typography>
            <Typography>Current application: {currentApplication}</Typography>
          </Box>
          <Typography textAlign="center" fontSize="2rem">
            Time spent in applications:
          </Typography>
          <TimeSpentBarChart />
          <Timeline />
        </>
      )}
    </Paper>
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
