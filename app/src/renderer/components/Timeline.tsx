import React from "react";
import { useSelector } from "react-redux";
import { RootState } from "../store/store";
import { CurrentSessionState } from "../store/currentSessionSlice";
import Chart from "react-google-charts";
import { Box } from "@mui/material";

function Timeline() {
  const currentSession = useSelector(
    (state: RootState) => state.currentSession
  );

  const mapChartData = (
    appEntries: CurrentSessionState["appVisitedEntries"]
  ): any[] => {
    const desc = [
      { type: "string", id: "Title" },
      { type: "date", id: "Start" },
      { type: "date", id: "End" },
    ];

    const data = [
      desc,
      ...appEntries
        .slice(0, appEntries.length - 2)
        .map((entry) => [
          entry.app_title,
          new Date(entry.start),
          new Date(entry.finish!),
        ]),
    ];
    return data;
  };

  const chartData = mapChartData(currentSession.appVisitedEntries);

  return (
    <Box className="md:container md:mx-auto">
      <Chart chartType="Timeline" data={chartData} />
    </Box>
  );
}

export default Timeline;
