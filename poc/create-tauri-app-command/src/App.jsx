import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { WebviewWindow, appWindow, getAll } from '@tauri-apps/api/window';


import "./App.css";

function App() {

  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  useEffect(() => {
    (async () => {
      
    })();
  }, []);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri! 2</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          console.log(2);
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
        <button type="button" onClick={
          async () => {
            console.log(1);
             const findWindow = new WebviewWindow('find', {
              url: '/index.html',
              width: 400,
              height: 400,
              alwaysOnTop: true,
            });
            findWindow.show();
          }
        }>open window</button>
        <button type="button" onClick={
          async () => {
            const allWindows = getAll();
            // iterate all Windows and find by label
            for (const window of allWindows) {
              if (window.label === 'find') {
                window.hide();
              }
            }

            //const searchWindow = appWindow.getByLabel('find');
            //searchWindow.hide();
          }
        }>hide window</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
