import { invoke } from "@tauri-apps/api";
import { Suspense } from "react";
import { Await, LoaderFunction, defer, useLoaderData } from "react-router-dom";

export const pricesLoader: LoaderFunction = async ({ params }) => {
  const link = JSON.parse(localStorage.getItem("items") || "[]")[params.id!];
  if (!link) throw new Error("Ссылка не найдена!");

  return defer({p: invoke<any>("parse", { link })});
};

const Item = () => {
  const v = useLoaderData();
  console.log(v);
  return (
    <section>
      <Suspense fallback={<div>Loading...</div>}>
        <Await resolve={v.p} errorElement={<p>Что-то не так!</p>}>
          {(v) => <div>{String(v)}</div>}
        </Await>
      </Suspense>
    </section>
  );
};

export default Item;
