import { Box } from "@mui/material";
import React, { useState, useEffect } from "react";
import {
  Cell,
  BarChart,
  Bar,
  XAxis,
  CartesianGrid,
  YAxis,
  Tooltip,
} from "recharts";

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
  async function fetchCurrentSessionStats() {
    const response = await fetch(
      "http://localhost:8000/api/session/current/statistics"
    );

    const sessionStats = (await response.json()) as SessionStats;

    const data = sessionStats.time_spent.map(([key, val]) => ({
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

    setSessionStats(sessionStats);
    setData(data);
    setChartData(chartData);
  }

  const [sessionStats, setSessionStats] = useState<SessionStats>();

  const [data, setData] =
    useState<{ name: string; ["time spent"]: number }[]>();
  const [chartData, setChartData] = useState<
    { name: string; ["time spent"]: number }[]
  >([]);

  useEffect(() => {
    fetchCurrentSessionStats();
  }, []);

  // TODO: better time formatting (not ms but let's sey hours and fractions of hours)
  // TODO: render each entry as separate <Bar> to have good legend?
  // TODO: add small color block in <XAxis>??

  console.log(chartData);
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

interface SessionStats {
  session: Session;
  time_spent: [string, number][];
}

interface Session {
  id: number;
  datetime: string;
}

export default TimeSpentBarChart;
