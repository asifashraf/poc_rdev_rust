import { useState, useEffect, useRef } from "react";
import {
  useRecoilState
} from 'recoil';
import socketState from "./state/socketState";
let websocketInstance = null;
const authCode = 'EdbKsUzjFHYNRmTAWqGClcBXgrZivLQhJoMItSbwEPaDnxOpfVuyXerHPksLOhBvXeUfzaCwIyRGtQJmNVblMnsjZdYKFrcPoAigXuhZWq';
function BasePage({children}) {

  const [socketConnected, setSocketConnected] = useRecoilState(socketState);
  const pageLoaded = useRef(false);
  const [ws, setWs] = useState(null);
  const [backendMessage, setBackendMessage] = useState('no message yet');
  const [messages, setMessages] = useState([]);

  useEffect(() => {
    (async () => {
      if (!pageLoaded.current) {
        pageLoaded.current = true;
        // Periodically check the connection
        connectSocket(setMessages, setBackendMessage, setWs, setSocketConnected);
        setInterval(function () {
          connectSocket(setMessages, setBackendMessage, setWs, setSocketConnected);
        }, 5000); // Check every 5 seconds
      }
    })();
  }, []);


  return (
    <div className="container">
      {socketConnected && <div>Listening</div>}
      {!socketConnected && <div>Not listening</div>}
      {children}
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


export default BasePage;
