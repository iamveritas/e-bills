import React, { createContext, useEffect, useState } from "react";

const MainContext = createContext();

function MainProvider({ children }) {
  const [current, setCurrent] = useState("home");
  const [amount, setAmount] = useState({ bill: 0, iou: 0, endors: 0 });
  const [currency, setCurrency] = useState("BTC");
  const [bills_list, setBillsList] = useState([]);
  const handlePage = (page) => {
    setCurrent(page);
  };
  const [peer_id, setPeerId] = useState({
    id: String,
  });
  // Set bills
  useEffect(() => {
    fetch("http://localhost:8000/bills/return")
      .then((res) => res.json())
      .then((data) => {
        console.log(data);
        setBillsList(data);
      })
      .catch((err) => {
        console.log(err.message);
      });
  }, []);
  // Set peer id
  useEffect(() => {
    fetch("http://localhost:8000/identity/peer_id/return")
      .then((res) => res.json())
      .then((data) => {
        setPeerId(data.id);
      })
      .catch((err) => {
        console.log(err.message);
      });
  }, []);
  console.log(amount);
  const amountCalculation = (items) => {
    if (peer_id == items.drawee.peer_id) {
      //   name = `${items?.drawee?.name} has to pay ${items?.payee?.name}`;
      setAmount((prev) => ({
        ...prev,
        iou: prev.iou + items.amount_numbers,
      }));
    } else if (peer_id == items.drawer.peer_id) {
      //   name = `${items.drawee.name} ${items.payee.name}`;
      setAmount((prev) => ({
        ...prev,
        endors: prev.endors + items.amount_numbers,
      }));
    } else if (peer_id == items.payee.peer_id) {
      //   name = `${items.drawee.name} ${items.payee.name}`;
      setAmount((prev) => ({
        ...prev,
        bill: prev.bill + items.amount_numbers,
      }));
    } else if (peer_id == items.endorsee.peer_id) {
      //   name = `${items.drawee.name} ${items.payee.name}`;
      setAmount((prev) => ({
        ...prev,
        bill: prev.bill + items.amount_numbers,
      }));
    }
  };
  useEffect(() => {
    if (peer_id && bills_list.length > 0) {
      bills_list.forEach((element) => {
        amountCalculation(element);
      });
    }
    return () => {};
  }, [peer_id, bills_list]);
  return (
    <MainContext.Provider
      value={{
        amount,
        bills_list,
        currency,
        peer_id,
        current,
        handlePage,
      }}
    >
      {children}
    </MainContext.Provider>
  );
}

export { MainContext, MainProvider };
