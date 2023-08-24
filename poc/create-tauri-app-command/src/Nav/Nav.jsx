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




  return (
    <div className="container">
      
      <h1>Navigation</h1>

      <button onClick={pasteTextViaClipboard} >paste_text_via_clipboard </button>

      <button onClick={setTextInClipboard} >set_text_in_clipboard </button>

      <button onClick={typeCharsOneByOne} >type_characters_one_by_one </button>
      <button onClick={writeSequence} >write_sequence</button>
      
      {
        messages.map((message, index) => {
          return <p key={index}>{message}</p>
        })
      }
      
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

function pasteTextViaClipboard () {
  setInterval(() => {
    websocketInstance.send(JSON.stringify({
      type: 'paste_text_via_clipboard', data: 'type this text bla bla bla... '
    }));
  }, 2000);
};

function setTextInClipboard () {
  websocketInstance.send(JSON.stringify({
    type: 'set_text_in_clipboard', data: 'set this text to clipbord... '
  }));
};


function typeCharsOneByOne(){
  setTimeout(() => {
    websocketInstance.send(JSON.stringify({
      type: 'type_characters_one_by_one', data: `start ❤️abcdefghijklmnopqrstuvwxyz
      ABCDEFGHIJKLMNOPQRSTUVWXYZ
      \`1234567890-=
      ~!@#$%^&*()_+
      []\\;',./{}|:"<>? before tab      after tab  two  spaces
      `
    }));
  }, 3000);
}

function writeSequence(){
  setTimeout(() => {
    websocketInstance.send(JSON.stringify({
      type: 'write_sequence', data: `start ❤️abcdefghijklmnopqrstuvwxyz
      ABCDEFGHIJKLMNOPQRSTUVWXYZ
      \`1234567890-=
      ~!@#$%^&*()_+
      []\\;',./{}|:"<>? before tab      after tab  two  spaces
      ====================
      This all happend via the sequence
      `
      
    }));
    
  }, 3000);
}


export default Nav;
