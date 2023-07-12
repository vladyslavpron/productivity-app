import React from "react";
import {
  Cell,
  BarChart,
  Bar,
  XAxis,
  CartesianGrid,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  Label,
} from "recharts";
import { RootState } from "../store/store";

import { useSelector } from "react-redux";
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
  // TODO: application icons

  const chartData = mapSessionTimeSpentData(currentSession.timePerApp);

  return (
    <ResponsiveContainer width="100%" aspect={4}>
      <BarChart title="Time spent in application" data={chartData}>
        <CartesianGrid strokeDasharray="3 4" />
        <Tooltip />
        <XAxis dataKey="name" />
        <YAxis padding={{ top: 20 }}>
          <Label
            value="Time spent (ms)"
            position="insideTop"
            dy={-7}
            dx={30}
            fill="black"
            fontSize="1.2em"
          />
        </YAxis>
        <Bar dataKey="time spent">
          {chartData?.map((entry, index) => (
            <Cell key={`cell-${index}`} fill={colors[index]}></Cell>
          ))}
        </Bar>
      </BarChart>
    </ResponsiveContainer>
  );
}

interface SessionTimeSpendMaped {
  name: string;
  "time spent": number;
}

export default TimeSpentBarChart;
