import { AppBar, Menu, MenuItem, Toolbar, Typography } from "@mui/material";
import React from "react";

function Navbar() {
  return (
    <AppBar position="static">
      <Toolbar>
        <MenuItem href="/">
          <Typography>Home</Typography>
        </MenuItem>
      </Toolbar>
    </AppBar>
  );
}

export default Navbar;
