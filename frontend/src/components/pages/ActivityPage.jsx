import React from "react";
import Header from "../sections/Header";
import TopDownHeading from "../elements/TopDownHeading";
import IconHolder from "../elements/IconHolder";
import hamburger from "../../assests/Vector.svg";
import bills from "../../assests/bills.svg";
import BillDetails from "../elements/BillDetails";

function ActivityPage({ bills_list }) {
  return (
    <div className="activity">
      <Header backHeader route="home" />
      <div className="head">
        <TopDownHeading upper="Drawer | All Payer | Payee" />
        <IconHolder icon={hamburger} />
      </div>
      <div className="bills">
        <BillDetails color="a3a3a3" data={bills_list} icon={bills} />
      </div>
    </div>
  );
}

export default ActivityPage;
