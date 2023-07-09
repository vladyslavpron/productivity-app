import { createSlice } from "@reduxjs/toolkit";
import type { PayloadAction } from "@reduxjs/toolkit";

export interface CurrentSessionState {
  session: Session | null;
  timePerApp: [string, number][];
  avgTimeInApp: number;
  totalTimeInApps: number;
}

const initialState: CurrentSessionState = {
  session: null,
  timePerApp: [],
  avgTimeInApp: 0,
  totalTimeInApps: 0,
};

export const currentSessionSlice = createSlice({
  name: "currentSession",
  initialState,
  reducers: {
    set: (state, action: PayloadAction<SessionStats>) => {
      state.session = action.payload.session;
      state.timePerApp = action.payload.time_per_app;
      state.avgTimeInApp = action.payload.avg_time_in_app;
      state.totalTimeInApps = action.payload.total_time_in_apps;
    },
  },
});

export const { set } = currentSessionSlice.actions;

export default currentSessionSlice.reducer;

export interface Session {
  id: number;
  datetime: string;
}

export interface SessionStats {
  session: Session;
  time_per_app: [string, number][];
  avg_time_in_app: number;
  total_time_in_apps: number;
}
