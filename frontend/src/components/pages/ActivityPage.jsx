import React from "react";
import Header from "../sections/Header";
import TopDownHeading from "../elements/TopDownHeading";
import IconHolder from "../elements/IconHolder";
import hamburger from "../../assests/Vector.svg";
import bills from "../../assests/bills.svg";
import BillDetails from "../elements/BillDetails";

function ActivityPage({ identity }) {
  const billsData = [
    {
      id: 1,
      name: "Bill on Bob",
      date: "16 April 2023",
      sign: "+",
      amount: "12.05",
      currency: "BTC",
    },
    {
      id: 2,
      name: "Billed By Dave",
      date: "06 May 2023",
      sign: "-",
      amount: "4.01",
      currency: "BTC",
    },
    {
      id: 3,
      name: "IOU to Charlie",
      date: "01 June 2023",
      sign: "-",
      amount: "3.01",
      currency: "BTC",
    },
    {
      id: 4,
      name: "IOU to Charlie",
      date: "04 June 2023",
      sign: "-",
      amount: "4.01",
      currency: "BTC",
    },
    {
      id: 5,
      name: "IOU to Charlie",
      date: "04 June 2023",
      sign: "-",
      amount: "4.01",
      currency: "BTC",
    },
    {
      id: 6,
      name: "IOU to Charlie",
      date: "04 June 2023",
      sign: "-",
      amount: "4.01",
      currency: "BTC",
    },
    {
      id: 7,
      name: "IOU to Charlie",
      date: "04 June 2023",
      sign: "-",
      amount: "4.01",
      currency: "BTC",
    },
    {
      id: 8,
      name: "IOU to Charlie",
      date: "04 June 2023",
      sign: "-",
      amount: "4.01",
      currency: "BTC",
    },
    {
      id: 9,
      name: "IOU to Charlie",
      date: "04 June 2023",
      sign: "-",
      amount: "4.01",
      currency: "BTC",
    },
  ];
  return (
    <div className="activity">
      <Header backHeader />
      <div className="head">
        <TopDownHeading upper="Drawer | All Payer | Payee" />
        <IconHolder icon={hamburger} />
      </div>
      <div className="bills">
        <BillDetails color="a3a3a3" data={billsData} icon={bills} />
      </div>
    </div>
  );
}

export default ActivityPage;
