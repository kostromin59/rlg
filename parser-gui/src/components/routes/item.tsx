import { invoke } from "@tauri-apps/api";
import { LoaderFunction, useLoaderData } from "react-router-dom";

export const pricesLoader: LoaderFunction = async ({ params }) => {
  const link = JSON.parse(localStorage.getItem("items") || "[]")[params.id!];
  if (!link) throw new Error("Ссылка не найдена!");

  let f = await invoke("parse", { link });
  return f;
};

const Item = () => {
  const v = useLoaderData();
  console.log(v);
  return <section>Item page</section>;
};

export default Item;
