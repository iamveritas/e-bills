import React from "react";

export default function RepayPage({ data, handlePage }) {
  return (
    <div className="Repay">
      <div className="col">
        <span>Send to</span>
        <span className="colored">{data.toOrder}</span>
      </div>
      <div className="inline">
        <span>the sum of </span>
        <span className="colored">
          {data.toSumCurrency} {data.toSumAmount}
        </span>
      </div>
      <div className="col mtt">
        <label htmlFor="wallet">Send from wallet:</label>
        <span className="select-opt">
          <select name="wallet" id="wallet">
            <option value="BTC" defaultValue="btc">
              Default BTC wallet
            </option>
            <option value="eterium">Ethereum</option>
          </select>
        </span>
        <button className="btn mtt" onClick={() => handlePage("bill")}>
          SIGN
        </button>
      </div>
    </div>
  );
}
