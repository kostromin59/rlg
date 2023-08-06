import { invoke } from "@tauri-apps/api";
import { Suspense } from "react";
import { Await, LoaderFunction, defer, useLoaderData } from "react-router-dom";
import Content from "../layouts/page/content";
import {
  Grid,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableRow,
  Typography,
} from "@mui/material";
import { Price } from "../../types/price";
import { open } from "@tauri-apps/api/shell";

export const pricesLoader: LoaderFunction = async ({ params }) => {
  const link = JSON.parse(localStorage.getItem("items") || "[]")[params.id!];
  if (!link) throw new Error("Ссылка не найдена!");

  return defer({ prices: invoke<Price[]>("parse", { link }) });
};

const Item = () => {
  const value = useLoaderData() as { prices: Price[] };
  return (
    <Content columns={7}>
      <Grid container padding={0} direction="column">
        <Suspense
          fallback={
            <Typography paddingY={3} textAlign="center" variant="h2">
              Загрузка...
            </Typography>
          }
        >
          <Await resolve={value.prices} errorElement={<p>Что-то не так!</p>}>
            {(prices: Price[]) => (
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>ID</TableCell>
                    <TableCell>Никнейм</TableCell>
                    <TableCell>Цена</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {prices
                    .sort((a, b) => a.price - b.price)
                    .map((price) => (
                      <TableRow
                        onClick={() =>
                          open(`https://rocket-league.com/trade/${price.id}`)
                        }
                        key={price.id}
                      >
                        <TableCell>{price.id}</TableCell>
                        <TableCell>{price.username}</TableCell>
                        <TableCell>{price.price}</TableCell>
                      </TableRow>
                    ))}
                </TableBody>
              </Table>
            )}
          </Await>
        </Suspense>
      </Grid>
    </Content>
  );
};

export default Item;
