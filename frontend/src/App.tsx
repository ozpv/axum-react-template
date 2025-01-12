import { MouseEvent, useState } from "react";
import { test_api } from "./api/test";

function App() {
  const [data, setData] = useState<String>("Click me to see a response");

  const on_click = (ev: MouseEvent<HTMLButtonElement>) => {
    ev.preventDefault();
    test_api().then((res) => {
      setData(res);
    });
  };

  return (
    <>
      <button onClick={on_click}>click me</button>
      <p>{data}</p>
    </>
  );
}

export default App;
