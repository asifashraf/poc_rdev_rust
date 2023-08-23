import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import reactLogo from "../assets/react.svg";
import { WebviewWindow, getAll } from '@tauri-apps/api/window';
import {
  useRecoilState,
} from 'recoil';
import socketState from "../state/socketState";

import "../App.css";

function App() {

  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [socketConnected, setSocketConnected] = useRecoilState(socketState);

  const openNav = async () => {
     const findWindow = new WebviewWindow('nav', {
      url: '/nav.html',
      width: 400,
      height: 400,
      alwaysOnTop: true,
    });
    findWindow.show();
  };

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  const hasRun = useRef(false);
  
  useEffect(() => {
    (async () =>{
      if (!hasRun.current) {
        hasRun.current = true;
        await openNav();
      }
    })();
  }, []);

  return (
    <div className="container">
      <h1>Dashboard</h1>

      {socketConnected && 'Socket connected'}
      {!socketConnected && 'Socket Disconnected'}

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
          openNav
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
