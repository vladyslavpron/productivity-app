import { createSlice } from "@reduxjs/toolkit";
import type { PayloadAction } from "@reduxjs/toolkit";

export interface CurrentSessionState {
  session: Session | null;
  time_spent: [string, number][];
}

const initialState: CurrentSessionState = {
  session: null,
  time_spent: [],
};

export const currentSessionSlice = createSlice({
  name: "currentSession",
  initialState,
  reducers: {
    set: (state, action: PayloadAction<SessionStats>) => {
      state.session = action.payload.session;
      state.time_spent = action.payload.time_spent;
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
  time_spent: [string, number][];
}
