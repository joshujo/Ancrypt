import "./App.css";
import { HashRouter as Router, Routes, Route } from "react-router";
import Login from "./OpeningPage/Login";

function App() {


  return (
    <Router>
      <Routes>
        <Route path="/" element={<Login/>} />
      </Routes>
    </Router>
  );
}

export default App;
