import { useState, useEffect, useRef } from "react";


function Nav() {
  const pageLoaded = useRef(false);
  const [ws, setWs] = useState(null);
  const [backendMessage, setBackendMessage] = useState('no message yet');
  const [messages, setMessages] = useState([]);
  useEffect(() => { 
    (async() => {

      
        const connectSocket = () => {
            
            // First time logic here. 
            console.log("Nav.jsx: loaded for first time only");
            const websocket = new WebSocket('ws://127.0.0.1:14705');
            websocket.onopen = () => {
                console.log('Connected to the WebSocket');
                websocket.send('EdbKsUzjFHYNRmTAWqGClcBXgrZivLQhJoMItSbwEPaDnxOpfVuyXerHPksLOhBvXeUfzaCwIyRGtQJmNVblMnsjZdYKFrcPoAigXuhZWq');
            };
    
            websocket.onmessage = (event) => {
                setMessages((prevMessages) => [...prevMessages, event.data]);
                setBackendMessage(event.data);
            };
    
            websocket.onerror = (error) => {
                console.error(`WebSocket Error: ${error}`);
            };
    
            websocket.onclose = () => {
                console.log('Disconnected from the WebSocket');
            };
    
            setWs(websocket);

          
        }

        // Periodically check the connection
        setInterval(function() {
          if (!ws || ws.readyState === WebSocket.CLOSED) {
              console.log("WebSocket is not connected. Attempting to reconnect...");
              connectSocket();
          }
        }, 5000); // Check every 5 seconds


        if (!pageLoaded.current) {
          pageLoaded.current = true;
          connectSocket();
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
        messages.map((message, index) => 
        {
            return <p key={index}>{message}</p>
        })
      }
      <button onClick={sendMessage} >send msg </button>
    </div>
  );
}
export default Nav;
