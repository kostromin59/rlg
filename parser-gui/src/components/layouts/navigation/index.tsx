import {
  Grid,
  List,
  ListItemButton,
  ListItemText,
  Typography,
} from "@mui/material";
import { Link } from "react-router-dom";

const Navigation = () => {
  return (
    <Grid item container xs={2} direction="column" padding={1}>
      <Typography align="center" component="h2" variant="h6">
        Навигация
      </Typography>
      <List>
        <ListItemButton component={Link} to="/items">
          <ListItemText primary="Вещи" />
        </ListItemButton>
        <ListItemButton component={Link} to="/items/add">
          <ListItemText primary="Добавить вещь" />
        </ListItemButton>
      </List>
    </Grid>
  );
};

export default Navigation;
