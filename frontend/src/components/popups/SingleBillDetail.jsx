import React, { useContext, useEffect, useState } from "react";
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
  const [singleBill, setSingleBill] = useState();
  const [singleBillChain, setSingleBillChain] = useState([]);
  useEffect(() => {
    fetch(`http://localhost:8000/bill/return/${item.name}`)
      .then((res) => res.json())
      .then((data) => {
        console.log(data);
        setSingleBill(data);
      })
      .catch((err) => {
        console.log(err.message);
      });
  }, []);
  useEffect(() => {
    fetch(`http://localhost:8000/bill/chain/return/${item.name}`)
      .then((res) => res.json())
      .then((data) => {
        console.log(data);
        setSingleBillChain(data.blocks);
      })
      .catch((err) => {
        console.log(err.message);
      });
  }, []);
  const buttons = [
    { isVisible: singleBill?.payed, name: "PAY", icon: iconPay },
    { isVisible: singleBill?.accepted, name: "ACCEPT", icon: iconAccept },
    { isVisible: singleBill?.endorsed, name: "ENDORSE", icon: iconEndorse },
    {
      isVisible: singleBill?.requested_to_accept,
      name: "REQUEST TO ACCEPT",
      icon: iconRTA,
    },
    {
      isVisible: singleBill?.requested_to_pay,
      name: "REQUEST TO Pay",
      icon: iconRTP,
    },
  ];

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
              <span className="detail">{singleBill?.maturity_date}</span>
            </span>
            <span className="block">
              <span className="accept-heading">to the order of </span>
              <span className="block detail ">{singleBill?.payee.name}</span>
            </span>
            <span className="block">
              <span className="accept-heading">the sum of </span>
              <span className="detail">
                {singleBill?.currency_code} {singleBill?.amount_numbers}
              </span>
            </span>
            <span className="block mt">
              <span className="accept-heading">Payer: </span>
              <span className="block detail">{singleBill?.drawee.name}</span>
            </span>
            <span className="block mt">
              <span className="accept-heading">Endorsed by: </span>
              <span className="block detail">
                {singleBillChain?.map((d, i) => (
                  <li key={i}>{d.bill_name.slice(0, 22)}...</li>
                ))}
              </span>
            </span>
          </div>
        </div>
        <div className="popup-btns">
          {buttons.map(({ isVisible, name, icon }, index) => {
            if (isVisible) {
              return (
                <button key={index} className="popup-btns-btn">
                  <img src={icon} /> <span>{name}</span>
                </button>
              );
            }
          })}
        </div>
      </div>
    </div>
  );
}
