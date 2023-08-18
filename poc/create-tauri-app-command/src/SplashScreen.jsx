import { useEffect } from "react";
import { useNavigate } from "react-router-dom";


import "./App.css";

function App() {
  const navigate = useNavigate();
  useEffect(() => {
    setTimeout(() => {
      navigate("/dashboard");
    }, 10);
  }, []);

  return (
    <div className="container">
      <h1>Welcome to Tauri! 2</h1>
      <h2>Please wait loading...</h2>
    </div>
  );
}

export default App;
