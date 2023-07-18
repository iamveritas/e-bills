import React from "react";

export default function AcceptPage({ data, handlePage }) {
  return (
    <div className="accept">
      <div className="accept-container">
        <div className="accept-container-content">
          <div className="block">
            <span className="accept-heading">Drawer:</span>
            <span className="detail block">Drawer Company, Paris</span>
          </div>
          <div className="block mt">
            <span className="block">
              <span className="accept-heading">Pay on </span>
              <span className="detail">{data.payonDate}</span>
            </span>
            <span className="block">
              <span className="accept-heading">to the order of </span>
              <span className="detail">{data.toOrder} </span>
            </span>
            <span className="block">
              <span className="accept-heading">the sum of </span>
              <span className="detail">
                {data.toSumCurrency} {data.toSumAmount}
              </span>
            </span>
          </div>
          <div className="block mttt">
            <span className="accept-heading">Accepted: </span>
            <span className="detail block">{data.drawee}</span>
          </div>
          <button className="btn mtt" onClick={() => handlePage("repay")}>
            SIGN
          </button>
        </div>
      </div>
    </div>
  );
}
