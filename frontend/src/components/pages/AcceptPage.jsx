import React from "react";

export default function AcceptPage({ data, handlePage }) {
  return (
    <div className="accept">
      <div className="accept-container">
        <div className="accept-container-content">
          <div className="block">
            <span className="accept-heading">Drawer:</span>
            <span className="detail block">{data.drawer_name}</span>
          </div>
          <div className="block">
            <span className="accept-heading">Drawee:</span>
            <span className="detail block">{data.drawee_name}</span>
          </div>
          <div className="block mt">
            <span className="block">
              <span className="accept-heading">Maturity date </span>
              <span className="detail">{data.maturity_date}</span>
            </span>
            <span className="block">
              <span className="accept-heading">to the order of </span>
              <span className="detail">{data.payee_name} </span>
            </span>
            <span className="block">
              <span className="accept-heading">the sum of </span>
              <span className="detail">
                {data.currency_code} {data.amount_numbers}
              </span>
            </span>
          </div>
          <button className="btn mtt" onClick={() => handlePage("repay")}>
            SIGN
          </button>
        </div>
      </div>
    </div>
  );
}
