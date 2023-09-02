import { useState, useEffect, useRef } from "react";
import {
  useRecoilState
} from 'recoil';
import socketState from "../state/socketState";
let websocketInstance = null;
import findNavWinByLabel from "./findNavWinByLabel";
import { authCode } from "../Components/commons";
import hideWin from "./hideWin";

const navWindow = findNavWinByLabel();

function Nav() {
  const [socketConnected, setSocketConnected] = useRecoilState(socketState);
  const pageLoaded = useRef(false);
  const messagesRef = useRef([]);
  const [ws, setWs] = useState(null);
  const [backendMessage, setBackendMessage] = useState('no message yet');
  const [messages, setMessages] = useState([]);
  const [isShown, setIsShown] = useState(false);
  const [hidwWinCount, setHideWinCount] = useState(0);
  const [charCount, setCharCount] = useState(0);

  useEffect(() => {
    (async () => {
      if (!pageLoaded.current) {
        pageLoaded.current = true;
        // Periodically check the connection
        connectSocket(setMessages, setBackendMessage, setWs, setSocketConnected, setCharCount);
        setInterval(function () {
          connectSocket(setMessages, setBackendMessage, setWs, setSocketConnected, setCharCount);
        }, 5000); // Check every 5 seconds
        // setInterval(function () {
        //   console.log('count messages', messagesRef.current.length);
        //   if(messagesRef.current.length > 0){
        //     navWindow.show();
        //   }else{
        //     navWindow.hide();
        //   }
        // }, 100); // Set visibility 
        // hide nav
        hideWin(setHideWinCount);
      }
    })();
  }, []);

  useEffect(() => {
    messagesRef.current = messages;
  }, [messages]);


  return (
    <div className="container">
      
      <h1>Chars {charCount}</h1>

      {socketConnected && <div>connected</div>}
      {!socketConnected && <div>disconnected</div>}

      <button onClick={() => {
        hideWin(setHideWinCount, true);
      }} >hide win {hidwWinCount}</button>
      <button onClick={() => {
        clearBuffer(setMessages, setBackendMessage, setCharCount);
      }} >clear</button>
      <button onClick={pasteTextViaClipboard} >paste_text_via_clipboard </button>
      <button onClick={setTextInClipboard} >set_text_in_clipboard </button>
      
      <button onClick={typeCharsOneByOne} >type_characters_one_by_one </button>
      better on win
      <button onClick={writeSequence} >write_sequence</button>
      better on mac 
      {
        messages.map((message, index) => {
          return <p key={index}>{message}</p>
        })
      }
      
    </div>
  );
}

const testText = `
if(true){
  return false;
}
❤️
ABCDEFGHIJKLMNOPQRSTUVWXYZ
abcdefghijklmnopqrstuvwxyz
\`1234567890-=
~!@#$%^&*()_+
[]\\;',./{}|:"<>? before tab  after tab  two  spaces
`;

function pasteTextViaClipboard () {
  setTimeout(() => {
    websocketInstance.send(JSON.stringify({
      type: 'paste_text_via_clipboard', data: `paste_text_via_clipboard: ` + testText
    }));
  }, 3000);
  
};

function setTextInClipboard () {
  websocketInstance.send(JSON.stringify({
    type: 'set_text_in_clipboard', data: 'set this text to clipbord... ' + testText
  }));
};


function typeCharsOneByOne(){
  setTimeout(() => {
    websocketInstance.send(JSON.stringify({
      type: 'type_characters_one_by_one', data: `type_characters_one_by_one` + testText
    }));
  }, 3000);
}

function writeSequence(){
  setTimeout(() => {
    websocketInstance.send(JSON.stringify({
      type: 'write_sequence', data: `write_sequence` + testText
      
    }));
    
  }, 3000);
}

function clearBuffer(setMessages, setBackendMessage, setCharCount){
  setMessages([]);
  setBackendMessage('');
  setCharCount(0);
}

function connectSocket(setMessages, setBackendMessage, setSocketInstance, setSocketConnected, setCharCount) {
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
    setMessages((prevMessages) => {
      let newArray = [...prevMessages, event.data];
      setCharCount(newArray.length);
      return newArray;
    });
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

export default Nav;
