import React from "react";
import IconHolder from "./IconHolder";

function BillDetails({ data, icon }) {
  return (
    <>
      {data.map((items, i) => {
        let sign = items.to_payee ? "+" : "-";
        return (
          <div key={i} className="home-container-bills-container">
            <IconHolder icon={icon} />
            <div className="details">
              <span className="name">{items.name}</span>
              <span className="date">{items.date_of_issue}</span>
            </div>
            <div className="currency-details">
              <div className={sign === "+" ? "amount" : "amount red"}>
                <span>{items.sign}</span>
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
