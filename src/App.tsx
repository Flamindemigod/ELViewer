// import { useState } from "react";
// import reactLogo from "./assets/react.svg";
// import { invoke } from "@tauri-apps/api/core";
import Header from "./components/Header.tsx";
import "./App.css";

function App() {
  // const [greetMsg, setGreetMsg] = useState("");
  // const [name, setName] = useState("");
  //
  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  return (
    <>
      <Header />
      <div className="relative flex-grow bg-red-500">
        <div>Main</div>
      </div>
      <div>Footer</div>
    </>
  );
}

export default App;
