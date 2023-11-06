import React, { useContext, useEffect } from "react";
import IconHolder from "./IconHolder";
import { MainContext } from "../../context/MainContext";
import SingleBillDetail from "../popups/SingleBillDetail";

const signCalculation = (peer_id, items) => {
  if (peer_id == items.drawee.peer_id) {
    //   name = `${items?.drawee?.name} has to pay ${items?.payee?.name}`;
    return "-";
  } else if (peer_id == items.drawer.peer_id) {
    //   name = `${items.drawee.name} ${items.payee.name}`;
    return "x";
  } else if (peer_id == items.payee.peer_id) {
    //   name = `${items.drawee.name} ${items.payee.name}`;
    return "+";
  } else if (peer_id == items.endorsee.peer_id) {
    //   name = `${items.drawee.name} ${items.payee.name}`;
    return "+";
  }
};
const namehandling = (peer_id, items) => {
  if (peer_id == items?.payer?.peer_id) {
    return items?.payee?.name;
  } else if (peer_id == items?.payee?.peer_id) {
    return items?.payer?.name;
  } else {
    return items?.payer?.name;
  }
};

function BillDetails({ data, icon }) {
  const { peer_id, popUp, showPopUp } = useContext(MainContext);
  console.log(peer_id, data);
  return (
    <>
      {data.map((items, i) => {
        let sign = signCalculation(peer_id, items);
        let name = namehandling(peer_id, items);
        return (
          <div
            key={i}
            onClick={() => showPopUp(true, <SingleBillDetail item={items} />)}
            className="home-container-bills-container"
          >
            <IconHolder icon={icon} />
            <div className="details">
              <span className="name">{name}</span>
              <span className="date">{items.date_of_issue}</span>
            </div>
            <div className="currency-details">
              <div
                className={
                  sign === "+"
                    ? "amount"
                    : sign === "x"
                    ? "amount grey"
                    : sign === "-"
                    ? "amount red"
                    : "amount grey"
                }
              >
                <span>{sign === "x" ? "" : sign}</span>
                <span>{items.amount_numbers}</span>
              </div>
              <span className="currency">{items.currency_code}</span>
            </div>
          </div>
        );
      })}
    </>
  );
}

export default BillDetails;
