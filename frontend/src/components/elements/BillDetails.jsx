import React, { useContext, useEffect } from "react";
import IconHolder from "./IconHolder";
import { MainContext } from "../../context/MainContext";

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

function BillDetails({ data, icon }) {
  const { peer_id } = useContext(MainContext);

  return (
    <>
      {data.map((items, i) => {
        let sign = signCalculation(peer_id, items);
        return (
          <div key={i} className="home-container-bills-container">
            <IconHolder icon={icon} />
            <div className="details">
              <span className="name">{items.name}</span>
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
                    : ""
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
