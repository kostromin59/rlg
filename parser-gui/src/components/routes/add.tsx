import { Button, Grid, TextField, Typography } from "@mui/material";
import Content from "../layouts/page/content";
import { FormEvent, useState } from "react";
import { useLocalStorage } from "usehooks-ts";
import { invoke } from "@tauri-apps/api";

const Add = () => {
  const [link, setLink] = useState("");
  const [tradeLink, setTradeLink] = useState("");
  const [profileLink, setProfileLink] = useState("");
  const [_, setItems] = useLocalStorage<string[]>("items", []);

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    setLink("");
    setItems((prev) => [...prev, link]);
  };

  const handleSubmitTrade = async (e: FormEvent) => {
    e.preventDefault();
    const links = await invoke<string[]>("parse_trade_links", {
      link: tradeLink,
    });
    setTradeLink("");
    setItems((prev) => [...prev, ...links]);
  };

  const handleSubmitProfile = async (e: FormEvent) => {
    e.preventDefault();
    console.log("EVENT");
    const links = await invoke<string[]>("parse_profile_links", {
      link: profileLink,
    });
    console.log(`LINKS ${links}`);
    setProfileLink("");
    setItems((prev) => [...prev, ...links]);
  };

  const isDisabled = !link.includes("https://rocket-league.com/");
  const isDisabledTrade = !tradeLink.includes(
    "https://rocket-league.com/trade/"
  );
  const isDisabledProfile = !profileLink.includes(
    "https://rocket-league.com/player/"
  );

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
        <Typography>Добавление по ссылке на трейд</Typography>
        <form onSubmit={handleSubmitTrade}>
          <Grid container gap={2}>
            <Grid xs={9} item>
              <TextField
                value={tradeLink}
                onChange={(e) => setTradeLink(e.target.value)}
                fullWidth
                label="Ссылка на трейд"
              />
            </Grid>
            <Button
              disabled={isDisabledTrade}
              type="submit"
              variant="contained"
            >
              Создать
            </Button>
          </Grid>
        </form>
        <Typography>Добавление по ссылке на профиль</Typography>
        <form onSubmit={handleSubmitProfile}>
          <Grid container gap={2}>
            <Grid xs={9} item>
              <TextField
                value={profileLink}
                onChange={(e) => setProfileLink(e.target.value)}
                fullWidth
                label="Ссылка на профиль"
              />
            </Grid>
            <Button
              disabled={isDisabledProfile}
              type="submit"
              variant="contained"
            >
              Создать
            </Button>
          </Grid>
        </form>
      </Grid>
    </Content>
  );
};

export default Add;
