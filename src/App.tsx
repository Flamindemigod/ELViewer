import "./App.css";
import Header from "./components/Header.tsx";
import Router from "./Router.tsx";

function App() {
  return (
    <>
      <Header />
      <div className="relative flex-grow bg-black/20 flex flex-col">
        <Router />
      </div>
      <div>Footer</div>
    </>
  );
}

export default App;
