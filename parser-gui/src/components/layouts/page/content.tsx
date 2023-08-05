import { Grid, Typography } from "@mui/material";
import { PropsWithChildren } from "react";

type Props = {
  title: string;
};

const Content = ({ title, children }: PropsWithChildren<Props>) => {
  return (
    <Grid xs={10} item>
      <Grid container direction="column" gap={2} padding={2}>
        <Typography variant="h3" component="h1">
          {title}
        </Typography>
        <Grid item>{children}</Grid>
      </Grid>
    </Grid>
  );
};

export default Content;
