import React, { createContext, useEffect, useState } from "react";

const MainContext = createContext();

function MainProvider({ children }) {
  const [current, setCurrent] = useState("");
  const [popUp, setPopUp] = useState({
    show: false,
    content: "",
  });
  const [amount, setAmount] = useState({ bill: 0, iou: 0, endors: 0 });
  const [currency, setCurrency] = useState("BTC");
  const [bills_list, setBillsList] = useState([]);
  const handlePage = (page) => {
    setCurrent(page);
  };
  const showPopUp = (show, content) => {
    setPopUp({
      show: show,
      content: content,
    });
  };
  const [peer_id, setPeerId] = useState({
    id: String,
  });

  const [refresh, setRefresh] = useState(false);
  const handleRefresh = () => {
    setRefresh(!refresh);
  };
  const [contacts, setContacts] = useState([]);
  // Set contacts
  useEffect(() => {
    fetch("http://localhost:8000/contacts/return")
      .then((res) => res.json())
      .then((data) => {
        setContacts(data);
      })
      .catch((err) => {
        console.log(err.message);
      });
  }, []);
  const [identity, setIdentity] = useState({
    name: "",
    date_of_birth: "",
    city_of_birth: "",
    country_of_birth: "",
    email: "",
    postal_address: "",
    public_key_pem: "",
    private_key_pem: "",
    bitcoin_public_key: "",
    bitcoin_private_key: "",
  });

  // Set identity
  useEffect(() => {
    fetch("http://localhost:8000/identity/return")
      .then((res) => res.json())
      .then((response) => {
        if (response.name !== "" && response.email !== "") {
          setIdentity(response);
          handlePage("home");
        } else {
          handlePage("identity");
        }
      })
      .catch((err) => {
        console.log(err.message);
        handlePage("identity");
      });
  }, [refresh]);

  // Set bills
  useEffect(() => {
    fetch("http://localhost:8000/bills/return")
      .then((res) => res.json())
      .then((data) => {
        let newData = data.sort(
          (a, b) => new Date(b.date_of_issue) - new Date(a.date_of_issue)
        );
        setBillsList(newData);
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
    setCurrency(items.currency_code);
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
        identity,
        amount,
        bills_list,
        refresh,
        contacts,
        handleRefresh,
        currency,
        peer_id,
        current,
        popUp,
        showPopUp,
        handlePage,
      }}
    >
      {children}
    </MainContext.Provider>
  );
}

export { MainContext, MainProvider };
