import {
  Button,
  Grid,
  Input,
  List,
  ListItemButton,
  ListItemText,
} from "@mui/material";
import { invoke } from "@tauri-apps/api";
import { MouseEvent, useEffect, useState } from "react";
import { useLocalStorage, useSessionStorage } from "usehooks-ts";
import { Item } from "../../../types/item";
import { Link, useNavigate } from "react-router-dom";

const buildInfo = (item: Item) => {
  return (
    <span>
      <span className="block">Цвет: {item.paint}</span>
      <span className="block">Сертификация: {item.certification}</span>
      <span className="block">Качество: {item.quality}</span>
      <span className="block">Серия: {item.series}</span>
      <span className="block">Тип вещи: {item.item_type}</span>
      <span className="block">Тип поиска: {item.search_type}</span>
      <span className="block">Платформа: {item.platform}</span>
    </span>
  );
};

const Items = () => {
  const [items, setItems] = useLocalStorage<string[]>("items", []);
  const [translatedItems, setTranslatedItems] = useState<
    { index: number; item: Item }[]
  >([]);
  const [search, setSearch] = useSessionStorage<string>("search", "");

  const navigate = useNavigate();

  useEffect(() => {
    invoke<Item[]>("links_to_cells", { links: items }).then((cells) => {
      const translated: { index: number; item: Item }[] = cells.map(
        (item, index) => ({ item, index })
      );
      setTranslatedItems(translated);
    });
  }, [items]);

  const clickHandle = (e: MouseEvent, index: number) => {
    e.preventDefault();

    setItems((prev) => {
      prev.splice(index, 1);
      return prev;
    });

    navigate("/items");
  };

  const filteredItems = translatedItems.filter((translated) => {
    if (!search) return true;
    const words = search.trim().split(" ");

    return words.every((word) =>
      Object.values(translated.item).some((field) =>
        field.toLowerCase().includes(word.toLowerCase())
      )
    );
  });

  return (
    <Grid item xs={3} height={"100vh"} overflow={"scroll"}>
      <Input
        placeholder="Поиск..."
        value={search}
        onChange={(e) => setSearch(e.target.value)}
      />
      <List>
        {(search.length >= 3 ? filteredItems : filteredItems.slice(0, 10)).map(
          (item) => (
            <ListItemButton
              key={item.index}
              component={Link}
              to={`/items/${item.index}`}
            >
              <Grid container>
                <ListItemText
                  primary={item.item.item}
                  secondary={buildInfo(item.item)}
                />
                <Button
                  onClick={(e) => clickHandle(e, item.index)}
                  variant="text"
                >
                  Удалить!
                </Button>
              </Grid>
            </ListItemButton>
          )
        )}
      </List>
    </Grid>
  );
};

export default Items;
