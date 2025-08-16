import "./App.css";
import { HashRouter as Router, Routes, Route } from "react-router";
import Login from "./OpeningPage/Login";
import Vault from "./Vault/Vault";

function App() {


  return (
    <Router>
      <Routes>
        <Route path="/" element={<Login/>} />
        <Route path="/Vault" element={<Vault/>} />
      </Routes>
    </Router>
  );
}

export default App;
