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
import AcceptPage from "../pages/AcceptPage";
import RepayPage from "../pages/RepayPage";
import EndorsePage from "../pages/EndorsePage";
import ReqAcceptPage from "../pages/ReqAcceptPage";
import ReqPaymentPage from "../pages/ReqPaymentPage";

export default function SingleBillDetail({ item }) {
  const { peer_id, showPopUp } = useContext(MainContext);
  const [singleBill, setSingleBill] = useState();
  //   const [singleBillChain, setSingleBillChain] = useState([]);
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
  //   useEffect(() => {
  //     fetch(`http://localhost:8000/bill/chain/return/${item.name}`)
  //       .then((res) => res.json())
  //       .then((data) => {
  //         console.log(data);
  //         setSingleBillChain(data.blocks);
  //       })
  //       .catch((err) => {
  //         console.log(err.message);
  //       });
  //   }, []);

  let payed = false;
  let accepted = false;
  let endorse = false;
  let req_to_pay = false;
  let req_to_acpt = false;
  let canMyPeerIdEndorse = peer_id == singleBill?.payee?.peer_id;
  let canMyPeerIdAccept = peer_id == singleBill?.drawee?.peer_id;
  let canMyPeerIdPay = peer_id == singleBill?.drawee?.peer_id;
  let canMyPeerIdReqToAccept = peer_id == singleBill?.payee?.peer_id;
  let canMyPeerIdReqToPay = peer_id == singleBill?.payee?.peer_id;

  if (
    !singleBill?.payed &&
    !singleBill?.accepted &&
    !singleBill?.pending &&
    canMyPeerIdAccept
  ) {
    accepted = true;
  }
  if (
    !singleBill?.payed &&
    !singleBill?.accepted &&
    !singleBill?.pending &&
    !singleBill?.endorse &&
    canMyPeerIdEndorse
  ) {
    endorse = true;
  }
  if (
    !singleBill?.payed &&
    !singleBill?.accepted &&
    !singleBill?.pending &&
    !singleBill?.requested_to_accept &&
    canMyPeerIdReqToAccept
  ) {
    req_to_acpt = true;
  }
  if (
    !singleBill?.payed &&
    !singleBill?.accepted &&
    !singleBill?.pending &&
    !singleBill?.requested_to_pay &&
    canMyPeerIdReqToPay
  ) {
    req_to_pay = true;
  }
  if (!singleBill?.payed && !singleBill?.pending && canMyPeerIdPay) {
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
      name: "REQUEST TO PAY",
      icon: iconRTP,
    },
  ];

  const handleApiCalling = async (name) => {
    switch (name) {
      case "PAY":
        showPopUp(true, <RepayPage data={singleBill} />);
        break;
      case "ACCEPT":
        showPopUp(true, <AcceptPage data={singleBill} />);
        break;
      case "ENDORSE":
        showPopUp(true, <EndorsePage data={singleBill} />);
        break;
      case "REQUEST TO ACCEPT":
        showPopUp(true, <ReqAcceptPage data={singleBill} />);
        break;
      case "REQUEST TO PAY":
        showPopUp(true, <ReqPaymentPage data={singleBill} />);
        break;
    }
  };

  return (
    <>
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
                {singleBill?.chain_of_blocks?.blocks?.map((d, i) => (
                  <li key={i}>{d.label.slice(0, 32)}...</li>
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
    </>
  );
}
