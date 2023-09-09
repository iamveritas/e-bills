import React, { createContext, useState } from "react";

const MainContext = createContext();
function MainProvider({ children }) {
  const [current, setCurrent] = useState("home");
  const handlePage = (page) => {
    setCurrent(page);
  };
  return (
    <MainContext.Provider value={{ current, handlePage }}>
      {children}
    </MainContext.Provider>
  );
}

export { MainContext, MainProvider };
