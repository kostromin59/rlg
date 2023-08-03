import { invoke } from "@tauri-apps/api";
import { createSignal } from "solid-js";

function App() {
  const [f, setF] = createSignal("");

  const parse = async () => {
    let data = await invoke<string>("greet", { link: "https://rocket-league.com/trade/7gxbnmq" });
    setF(data);
  };

  return (
    <div>
      <h1>Parser</h1>
      <h2>{f()}</h2>
      <button onClick={parse}>Parse</button>
    </div>
  );
}

export default App;
