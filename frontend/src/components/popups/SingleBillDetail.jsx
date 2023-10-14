import React, { useContext } from "react";
import closeBtn from "../../assests/close-btn.svg";
import attachment from "../../assests/attachment.svg";
import { MainContext } from "../../context/MainContext";
import TopDownHeading from "../elements/TopDownHeading";
import IconHolder from "../elements/IconHolder";

import iconPay from "../../assests/pay.svg";
import iconAccept from "../../assests/accept.svg";
import iconEndorse from "../../assests/endorse.svg";
import iconRTA from "../../assests/reqToPay.svg";
import iconRTP from "../../assests/reqToAccept.svg";

export default function SingleBillDetail({ item }) {
  const { showPopUp } = useContext(MainContext);
  const buttons = [
    { name: "PAY", icon: iconPay },
    { name: "ACCEPT", icon: iconAccept },
    { name: "ENDORSE", icon: iconEndorse },
    { name: "REQUEST TO ACCEPT", icon: iconRTA },
    { name: "REQUEST TO Pay", icon: iconRTP },
  ];
  console.log(item);
  return (
    <div className="popup">
      <div className="popup-head">
        <span className="popup-head-title">
          {item.place_of_drawing}, {item.date_of_issue}
        </span>
        <img
          className="close-btn"
          onClick={() => showPopUp(false, "")}
          src={closeBtn}
        />
      </div>
      <div className="popup-body">
        <div className="popup-body-inner">
          <div className="head">
            <TopDownHeading upper="Against this" lower="Bill Of Exchange" />
            <IconHolder icon={attachment} />
          </div>
          <div className="block mt">
            <span className="block">
              <span className="accept-heading">pay on </span>
              <span className="detail">16 May 2023</span>
            </span>
            <span className="block">
              <span className="accept-heading">to the order of </span>
              <span className="block detail ">Payee Company, NY</span>
            </span>
            <span className="block">
              <span className="accept-heading">the sum of </span>
              <span className="detail">BTC 3.125</span>
            </span>
            <span className="block mt">
              <span className="accept-heading">Payer: </span>
              <span className="block detail">Drawee Company, Vienna</span>
            </span>
            <span className="block mt">
              <span className="accept-heading">Endorsed by: </span>
              <span className="block detail">
                <li>Payee Company, NY</li>
                <li>Payee Company, NY</li>
              </span>
            </span>
          </div>
        </div>
        <div className="popup-btns">
          {buttons.map(({ name, icon }, index) => {
            return (
              <button key={index} className="popup-btns-btn">
                <img src={icon} /> <span>{name}</span>
              </button>
            );
          })}
        </div>
      </div>
    </div>
  );
}
