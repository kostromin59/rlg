import {
  Button,
  Grid,
  List,
  ListItemButton,
  ListItemText,
} from "@mui/material";
import { invoke } from "@tauri-apps/api";
import { MouseEvent, useEffect, useState } from "react";
import { useLocalStorage } from "usehooks-ts";
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
  const [translatedItems, setTranslatedItems] = useState<Item[]>([]);

  const navigate = useNavigate();

  useEffect(() => {
    invoke<Item[]>("links_to_cells", { links: items }).then((cells) => {
      setTranslatedItems(cells);
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

  return (
    <Grid item xs={3} height={"100vh"} overflow={"scroll"}>
      <List>
        {translatedItems.map((item, index) => (
          <ListItemButton key={index} component={Link} to={`/items/${index}`}>
            <Grid container>
              <ListItemText primary={item.item} secondary={buildInfo(item)} />
              <Button onClick={(e) => clickHandle(e, index)} variant="text">
                Удалить!
              </Button>
            </Grid>
          </ListItemButton>
        ))}
      </List>
    </Grid>
  );
};

export default Items;
