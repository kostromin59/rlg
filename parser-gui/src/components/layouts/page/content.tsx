import { Grid, Typography } from "@mui/material";
import { PropsWithChildren } from "react";

type Props = {
  title?: string;
  columns?: number;
};

const Content = ({
  title,
  columns = 10,
  children,
}: PropsWithChildren<Props>) => {
  return (
    <Grid xs={columns} item>
      <Grid container direction="column" gap={2} padding={0}>
        {title && (
          <Typography variant="h3" component="h1">
            {title}
          </Typography>
        )}
        <Grid item padding={0} maxHeight={"100vh"} overflow="scroll">
          {children}
        </Grid>
      </Grid>
    </Grid>
  );
};

export default Content;
