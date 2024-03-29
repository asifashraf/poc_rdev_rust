import React from "react";
import ReactDOM from "react-dom/client";
import {
  createBrowserRouter,
  RouterProvider
} from "react-router-dom";
import queryString from 'query-string';
import "./styles.css";
import SplashScreen from "./SplashScreen";
import Dashboard from "./Dashboard/Dashboard";
import Nav from "./Nav/Nav";
import {
  RecoilRoot,
  atom,
  selector,
  useRecoilState,
  useRecoilValue,
} from 'recoil';
const parsed = queryString.parse(location.search);
console.log('query string', parsed);
const router = createBrowserRouter([
  {
    path: "/",
    element: <SplashScreen />,
  },
  {
    path: "/dashboard",
    element: <Dashboard />,
  },
  {
    path: "/nav.html",
    element: <Nav />,
  },
]);
ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <RecoilRoot>
      <RouterProvider router={router} />
    </RecoilRoot>
    
  </React.StrictMode>,
);
