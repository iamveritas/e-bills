import React, { useContext, useEffect, useState } from "react";

// Pages
import IssuePage from "./components/pages/IssuePage";
import AcceptPage from "./components/pages/AcceptPage";
import RepayPage from "./components/pages/RepayPage";
import Bill from "./components/pages/Bill";
import { MainContext, MainProvider } from "./context/MainContext";
import HomePage from "./components/pages/HomePage";
import ActivityPage from "./components/pages/ActivityPage";
export default function App() {
  const { current } = useContext(MainContext);
  const [data, setData] = useState({
    maturity_date: "",
    payee_name: "",
    currency_code: "sats",
    amount_numbers: "",
    drawee_name: "",
    drawer_name: "",
    place_of_drawing: "",
    place_of_payment: "",
    bill_jurisdiction: "",
    date_of_issue: "",
    language: "en",
  });
  const [identity, setIdentity] = useState({
    name: String,
    date_of_birth: String,
    city_of_birth: String,
    country_of_birth: String,
    email: String,
    postal_address: String,
    public_key_pem: String,
    private_key_pem: String,
    bitcoin_public_key: String,
    bitcoin_private_key: String,
  });
  // Set identity
  useEffect(() => {
    fetch("http://localhost:8000/identity/return")
      .then((res) => res.json())
      .then((data) => {
        console.log(data);
        setIdentity(data);
      })
      .catch((err) => {
        console.log(err.message);
      });
  }, []);
  const [contacts, setContacts] = useState([]);
  // Set contacts
  useEffect(() => {
    fetch("http://localhost:8000/contacts/return")
      .then((res) => res.json())
      .then((data) => {
        console.log(data);
        setContacts(data);
      })
      .catch((err) => {
        console.log(err.message);
      });
  }, []);
  const changeHandle = (e) => {
    let value = e.target.value;
    let name = e.target.name;
    setData({ ...data, [name]: value });
  };
  const activePage = () => {
    switch (current) {
      case "home":
        return <HomePage />;
      case "issue":
        return <IssuePage contacts={contacts} identity={identity} data={data} changeHandle={changeHandle} />;
      case "accept":
        return <AcceptPage identity={identity} data={data} />;
      case "repay":
        return <RepayPage contacts={contacts} identity={identity} data={data} />;
      case "activity":
        return <ActivityPage />;
      case "bill":
        return <Bill identity={identity} data={data} />;
    }
  };

  return <>{activePage()}</>;
}
