import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import reactLogo from "../assets/react.svg";
import { WebviewWindow, getAll } from '@tauri-apps/api/window';

import {
  useRecoilState
} from 'recoil';
import socketState from "../state/socketState";
import "../App.css";

import { authCode } from "../Components/commons";
let websocketInstance = null;
function App() {

  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  
  const [socketConnected, setSocketConnected] = useRecoilState(socketState);
  const [ws, setWs] = useState(null);
  const [backendMessage, setBackendMessage] = useState('no message yet');
  const [messages, setMessages] = useState([]);

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
    (async () => {
      if (!hasRun.current) {
        hasRun.current = true;
        await openNav();
        connectSocket(setMessages, setBackendMessage, setWs, setSocketConnected);
        setInterval(function () {
          connectSocket(setMessages, setBackendMessage, setWs, setSocketConnected);
        }, 5000); // Check every 5 seconds
      }
    })();
  }, []);

  return (
    <div className="container">

      {socketConnected && <div>connected</div>}
      {!socketConnected && <div>disconnected</div>}

      <form
        className="row"
        onSubmit={(e) => {
          console.log(2);
          e.preventDefault();
          greet();
        }}
      >
        <button type="button" onClick={
          openNav
        }>open window</button>
        <button type="button" onClick={
          async () => {
            const allWindows = getAll();
            // iterate all Windows and find by label
            for (const window of allWindows) {
              if (window.label === 'nav') {
                window.hide();
              }
            }

            //const searchWindow = appWindow.getByLabel('find');
            //searchWindow.hide();
          }
        }>hide window</button>
      </form>




    </div>
  );
}
function connectSocket(setMessages, setBackendMessage, setSocketInstance, setSocketConnected) {
  if (websocketInstance?.readyState === WebSocket.OPEN) {
    setSocketConnected(true);
    console.log('Nav.jsx: already connected to the WebSocket');
    return;
    
  }
  console.log('Nav.jsx: trying to connect to the WebSocket');
  setSocketConnected(false);
  
  // First time logic here. 
  console.log("Nav.jsx: loaded for first time only");
  websocketInstance = new WebSocket('ws://127.0.0.1:14705');
  websocketInstance.onopen = () => {
    setSocketConnected(true);
    console.log('Connected to the WebSocket');
    websocketInstance.send(JSON.stringify({
      type: 'auth', data: authCode
    }));
  };
  websocketInstance.onmessage = (event) => {
    setSocketConnected(true);
    setMessages((prevMessages) => [...prevMessages, event.data]);
    setBackendMessage(event.data);
  };
  websocketInstance.onerror = (error) => {
    console.error(`WebSocket Error: ${error}`);
    setSocketConnected(false);
  };
  websocketInstance.onclose = () => {
    console.log('Disconnected from the WebSocket');
    setSocketConnected(false);
  };
  setSocketInstance(websocketInstance);
}
export default App;
