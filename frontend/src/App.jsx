import React, { useState } from "react";

import attachment from "./assests/attachment.svg";
// components
import Header from "./components/sections/Header";
import UniqueNumber from "./components/sections/UniqueNumber";
import TopDownHeading from "./components/elements/TopDownHeading";
import IconHolder from "./components/elements/IconHolder";
// Pages
import IssuePage from "./components/pages/IssuePage";
import AcceptPage from "./components/pages/AcceptPage";
import RepayPage from "./components/pages/RepayPage";
import Bill from "./components/pages/Bill";
export default function App() {
  const [current, setCurrent] = useState("issue");
  const [data, setData] = useState({
    payon: "",
    payonDate: "",
    toOrder: "",
    toSumCurrency: "",
    toSumAmount: "",
    drawee: "",
  });

  const changeHandle = (e) => {
    let value = e.target.value;
    let name = e.target.name;
    setData({ ...data, [name]: value });
  };

  const handlePage = (page) => {
    setCurrent(page);
  };

  const activePage = () => {
    switch (current) {
      case "issue":
        return (
          <IssuePage
            data={data}
            changeHandle={changeHandle}
            handlePage={handlePage}
          />
        );
      case "accept":
        return <AcceptPage data={data} handlePage={handlePage} />;
      case "repay":
        return <RepayPage data={data} handlePage={handlePage} />;
      case "bill":
        return <Bill data={data} handlePage={handlePage} />;
    }
  };

  return (
    <>
      {current !== "bill" && (
        <>
          <Header title="Issue" />
          <UniqueNumber UID="001" date="16-Feb-2023" />
          <div className="head">
            <TopDownHeading upper="Against this" lower="Bill Of Exchange" />
            <IconHolder icon={attachment} />
          </div>
        </>
      )}
      {activePage()}
    </>
  );
}
