import React from "react";
import IconHolder from "./IconHolder";

function BillDetails({ data, color, icon }) {
  return (
    <>
      {data.map((items, i) => {
        return (
          <div key={i} className="home-container-bills-container">
            <IconHolder icon={icon} />
            <div className="details">
              <span className="name">{items.name}</span>
              <span className="date">{items.date}</span>
            </div>
            <div className="currency-details">
              <div className={items?.sign == "+" ? "amount" : "amount red"}>
                <span>{items.sign}</span>
                <span>{items.amount}</span>
              </div>
              <span className="currency">{items.currency}</span>
            </div>
          </div>
        );
      })}
    </>
  );
}

export default BillDetails;
