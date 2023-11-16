import React, { useContext } from "react";
import Header from "../sections/Header";
import TopDownHeading from "../elements/TopDownHeading";
import IconHolder from "../elements/IconHolder";
import attachment from "../../assests/attachment.svg";
import { MainContext } from "../../context/MainContext";
import copy from "../../assests/copy.svg";

export default function RepayPage({ data }) {
  return (
    <div className="Repay">
      <Header title="Pay" />
      {/*<UniqueNumber UID="001" date="16-Feb-2023" />*/}
      <div className="head">
        <TopDownHeading upper="Against this" lower="Bill Of Exchange" />
        <IconHolder icon={attachment} />
      </div>
      <div className="col">
        <span>Send to</span>
        <span className="colored">{data.payee.name}</span>
      </div>
      <div className="inline">
        <span>the sum of </span>
        <span className="colored" style={{ textTransform: "uppercase" }}>
          {data.currency_code} {data.amount_numbers}
        </span>
      </div>
      <div className="col mtt">
        <label htmlFor="wallet">Link to pay:</label>
        <span
          className="select-opt"
          onClick={() => {
            navigator.clipboard.writeText(data.link_to_pay);
          }}
        >
          {data.link_to_pay?.slice(0, 20)}...{" "}
          <img
            style={{
              width: "5vw",
              height: "5vw",
              display: "inline-block",
              verticalAlign: "middle",
            }}
            src={copy}
          />
        </span>
      </div>
    </div>
  );
}
