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
  const { peer_id, showPopUp } = useContext(MainContext);
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

  let payed = false;
  let accepted = false;
  let endorse = false;
  let req_to_pay = false;
  let req_to_acpt = false;
  let notEndorsed = peer_id == singleBill?.payee?.peer_id;
  let isAccepted = peer_id == singleBill?.payee?.peer_id;
  let isEndorsed = peer_id == singleBill?.endorsee?.peer_id;
  let isPayed = peer_id == singleBill?.drawee?.peer_id;

  if (
    !singleBill?.payed &&
    !singleBill?.accepted &&
    !singleBill?.pending &&
    isAccepted
  ) {
    accepted = true;
  }
  if (
    !singleBill?.payed &&
    !singleBill?.accepted &&
    !singleBill?.pending &&
    !singleBill?.endorse &&
    notEndorsed &&
    isEndorsed
  ) {
    endorse = true;
  }
  if (
    !singleBill?.payed &&
    !singleBill?.accepted &&
    !singleBill?.pending &&
    !singleBill?.requested_to_accept &&
    notEndorsed &&
    isEndorsed
  ) {
    req_to_acpt = true;
  }
  if (
    !singleBill?.payed &&
    !singleBill?.accepted &&
    !singleBill?.pending &&
    !singleBill?.requested_to_pay &&
    notEndorsed &&
    isEndorsed
  ) {
    req_to_pay = true;
  }
  if (!singleBill?.payed && !singleBill?.pending && isPayed) {
    payed = true;
  }

  const buttons = [
    { isVisible: payed, name: "PAY", icon: iconPay },
    { isVisible: accepted, name: "ACCEPT", icon: iconAccept },
    { isVisible: endorse, name: "ENDORSE", icon: iconEndorse },
    {
      isVisible: req_to_acpt,
      name: "REQUEST TO ACCEPT",
      icon: iconRTA,
    },
    {
      isVisible: req_to_pay,
      name: "REQUEST TO Pay",
      icon: iconRTP,
    },
  ];

  const handleApiCalling = async (name) => {
    switch (name) {
      case "PAY":
        await fetch("", {})
          .then((res) => console.log(res))
          .catch((err) => console.log(err));
      case "ACCEPT":
        await fetch("", {})
          .then((res) => console.log(res))
          .catch((err) => console.log(err));
      case "ENDORSE":
        await fetch("", {})
          .then((res) => console.log(res))
          .catch((err) => console.log(err));
      case "REQUEST TO ACCEPT":
        await fetch("", {})
          .then((res) => console.log(res))
          .catch((err) => console.log(err));
      case "REQUEST TO Pay":
        await fetch("", {})
          .then((res) => console.log(res))
          .catch((err) => console.log(err));
    }
  };

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
                <button
                  key={index}
                  onClick={() => handleApiCalling(name)}
                  className="popup-btns-btn"
                >
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
