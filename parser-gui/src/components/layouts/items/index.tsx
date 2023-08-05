import { Grid } from "@mui/material";
import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { useLocalStorage } from "usehooks-ts";

const Items = () => {
  const [items, setItems] = useLocalStorage<string[]>("items", []);
  const [translatedItems, setTranslatedItems] = useState<any[]>([]);

  useEffect(() => {
    invoke<any[]>("links_to_cells", { links: items })
      .then((cells) => {
        setTranslatedItems(cells);
      })
      .catch((e) => { });
  }, [items]);
  console.log(translatedItems);

  return (
    <Grid item xs={3}>
      {translatedItems.map((item) => <div>{item.item}</div>)}
    </Grid>
  );
};

export default Items;
