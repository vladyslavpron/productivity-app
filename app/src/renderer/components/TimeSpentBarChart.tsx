import { Box } from "@mui/material";
import React, { useState, useEffect } from "react";
import { Root } from "react-dom/client";
import {
  Cell,
  BarChart,
  Bar,
  XAxis,
  CartesianGrid,
  YAxis,
  Tooltip,
} from "recharts";
import { RootState } from "../store/store";

import { useSelector, useDispatch } from "react-redux";
import { SessionStats } from "../store/currentSessionSlice";

const colors = [
  "#ea5545",
  "#f46a9b",
  "#ef9b20",
  "#edbf33",
  "#ede15b",
  "#bdcf32",
  "#87bc45",
  "#27aeef",
  "#b33dc6",
];

const FULL_DISPLAY_STATS_COUNT = 5;

function TimeSpentBarChart() {
  const currentSession = useSelector(
    (state: RootState) => state.currentSession
  );

  const mapSessionTimeSpentData = (
    timeSpent: SessionStats["time_per_app"]
  ): SessionTimeSpendMaped[] => {
    const data = timeSpent.map(([key, val]) => ({
      name: key,
      "time spent": val,
    }));

    const chartData = [];
    let i = 0;
    while (data[i] && i < FULL_DISPLAY_STATS_COUNT) {
      chartData.push(data[i]);
      i++;
    }

    let count = 0;
    while (data[i]) {
      count += data[i]["time spent"];
      i++;
    }

    chartData.push({ name: "others", ["time spent"]: count });

    return chartData;
  };

  // TODO: better time formatting (not ms but let's sey hours and fractions of hours)
  // TODO: render each entry as separate <Bar> to have good legend?
  // TODO: add small color block in <XAxis>??
  const chartData = mapSessionTimeSpentData(currentSession.timePerApp);

  return (
    <Box width="100%" height="100%" margin="50px">
      <BarChart
        title="Time spend in application"
        width={1000}
        height={500}
        data={chartData}
      >
        <CartesianGrid strokeDasharray="3 3" />
        <Tooltip />
        <XAxis dataKey="name" />
        <YAxis />
        <Bar dataKey="time spent" label>
          {chartData?.map((entry, index) => (
            <Cell key={`cell-${index}`} fill={colors[index]}></Cell>
          ))}
        </Bar>
      </BarChart>
    </Box>
  );
}

interface SessionTimeSpendMaped {
  name: string;
  "time spent": number;
}

export default TimeSpentBarChart;
