import { useState, useEffect, useRef } from "react";
import {
  useRecoilState
} from 'recoil';
import socketState from "../state/socketState";
let websocketInstance = null;
const authCode = 'EdbKsUzjFHYNRmTAWqGClcBXgrZivLQhJoMItSbwEPaDnxOpfVuyXerHPksLOhBvXeUfzaCwIyRGtQJmNVblMnsjZdYKFrcPoAigXuhZWq';
function Nav() {

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
        connectSocket(setMessages, setBackendMessage, setWs);
        setInterval(function () {
          connectSocket(setMessages, setBackendMessage, setWs);
        }, 5000); // Check every 5 seconds
      }
    })();
  }, []);


  const sendMessage = () => {
    ws.send('hi nb');
  };

  return (
    <div className="container">
      <h1>Navigation</h1>
      {
        messages.map((message, index) => {
          return <p key={index}>{message}</p>
        })
      }
      <button onClick={sendMessage} >send msg </button>
    </div>
  );
}



function connectSocket(setMessages, setBackendMessage, setSocketInstance) {
  if (websocketInstance?.readyState === WebSocket.OPEN) return;
  
  // First time logic here. 
  console.log("Nav.jsx: loaded for first time only");
  websocketInstance = new WebSocket('ws://127.0.0.1:14705');
  websocketInstance.onopen = () => {
    console.log('Connected to the WebSocket');
    websocketInstance.send(JSON.stringify({
      type: 'auth', data: authCode
    }));
  };
  websocketInstance.onmessage = (event) => {
    setMessages((prevMessages) => [...prevMessages, event.data]);
    setBackendMessage(event.data);
  };
  websocketInstance.onerror = (error) => {
    console.error(`WebSocket Error: ${error}`);
  };
  websocketInstance.onclose = () => {
    console.log('Disconnected from the WebSocket');
  };
  setSocketInstance(websocketInstance);
}
export default Nav;
