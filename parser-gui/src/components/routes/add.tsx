import { Button, Grid, TextField, Typography } from "@mui/material";
import Content from "../layouts/page/content";
import { FormEvent, useState } from "react";
import { useLocalStorage } from "usehooks-ts";

const Add = () => {
  const [link, setLink] = useState("");
  const [_, setItems] = useLocalStorage<string[]>("items", []);

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    console.log("Submitted!", link);
    setItems((prev) => [...prev, link]);
  };

  const isDisabled = !link.includes("https://rocket-league.com/")

  return (
    <Content title="Добавление вещи">
      <Grid container direction="column" gap={2}>
        <Typography>
          Для создания введите ссылку. Зайдите на сайт, укажите в фильтрах
          нужную вещь и её параметры и скопируйте ссылку.
        </Typography>
        <form onSubmit={handleSubmit}>
          <Grid container gap={2}>
            <Grid xs={9} item>
              <TextField
                value={link}
                onChange={(e) => setLink(e.target.value)}
                fullWidth
                label="Ссылка на поиск вещи"
              />
            </Grid>
            <Button disabled={isDisabled} type="submit" variant="contained">
              Создать
            </Button>
          </Grid>
        </form>
      </Grid>
    </Content>
  );
};

export default Add;
