import React, { createContext, useEffect, useState } from "react";

const MainContext = createContext();

function MainProvider({ children }) {
  const [current, setCurrent] = useState("home");
  const handlePage = (page) => {
    setCurrent(page);
  };
  const [peer_id, setPeerId] = useState({
    id: String,
  });
  // Set peer id
  useEffect(() => {
    fetch("http://localhost:8000/identity/peer_id/return")
      .then((res) => res.json())
      .then((data) => {
        setPeerId(data.id);
      })
      .catch((err) => {
        console.log(err.message);
      });
  }, []);
  return (
    <MainContext.Provider value={{ peer_id, current, handlePage }}>
      {children}
    </MainContext.Provider>
  );
}

export { MainContext, MainProvider };
