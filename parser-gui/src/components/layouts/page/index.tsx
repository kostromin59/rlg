import { Outlet } from "react-router-dom";
import Navigation from "../navigation";
import { Grid } from "@mui/material";

const Page = () => {
  return (
    <div>
      <Grid container >
        <Navigation />
        <Outlet />
      </Grid>
    </div>
  );
};

export default Page;
