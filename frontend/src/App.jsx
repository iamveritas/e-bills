import React, { useContext, useEffect, useState } from "react";

import IssuePage from "./components/pages/IssuePage";
import AcceptPage from "./components/pages/AcceptPage";
import RepayPage from "./components/pages/RepayPage";
import Bill from "./components/pages/Bill";
import { MainContext } from "./context/MainContext";
import HomePage from "./components/pages/HomePage";
import ActivityPage from "./components/pages/ActivityPage";
import EndorsePage from "./components/pages/EndorsePage";
import ReqPaymentPage from "./components/pages/ReqPaymentPage";
import ReqAcceptPage from "./components/pages/ReqAcceptPage";
import IdentityPage from "./components/pages/IdentityPage";
import SettingPage from "./components/pages/SettingPage";

export default function App() {
  const { current, handlePage } = useContext(MainContext);
  // Set data for bill issue
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
    drawer_is_payee: false,
    drawer_is_drawee: false,
  });

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
  const [identityRefresh, setIdentityRefresh] = useState(false);
  const handleRefresh = () => {
    setIdentityRefresh(!identityRefresh);
  };
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
  }, [identityRefresh]);

  const [contacts, setContacts] = useState([]);
  // Set contacts
  useEffect(() => {
    fetch("http://localhost:8000/contacts/return")
      .then((res) => res.json())
      .then((data) => {
        console.log("contacts/return: ", data);
        setContacts(data);
      })
      .catch((err) => {
        console.log(err.message);
      });
  }, []);

  const [operation_codes, setOperationCodes] = useState([]);
  // Set operation codes
  useEffect(() => {
    fetch("http://localhost:8000/opcodes/return")
      .then((res) => res.json())
      .then((data) => {
        console.log("/opcodes/return: ", data);
        setOperationCodes(data);
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
  const handleChangeDrawerIsPayee = (e) => {
    let value = !data.drawer_is_payee;
    let name = e.target.name;
    setData({ ...data, [name]: value });
  };
  const handleChangeDrawerIsDrawee = (e) => {
    let value = !data.drawer_is_drawee;
    let name = e.target.name;
    setData({ ...data, [name]: value });
  };

  const activePage = () => {
    switch (current) {
      case "identity":
        return <IdentityPage refresh={handleRefresh} identity={identity} />;
      case "home":
        return <HomePage />;
      case "activity":
        return <ActivityPage />;
      case "reqaccept":
        return <ReqAcceptPage />;
      case "reqpayment":
        return <ReqPaymentPage />;
      case "endorse":
        return <EndorsePage />;
      case "issue":
        return (
          <IssuePage
            handleChangeDrawerIsDrawee={handleChangeDrawerIsDrawee}
            contacts={contacts}
            identity={identity}
            data={data}
            changeHandle={changeHandle}
            handleChangeDrawerIsPayee={handleChangeDrawerIsPayee}
          />
        );
      case "accept":
        return <AcceptPage identity={identity} data={data} />;
      case "repay":
        return (
          <RepayPage contacts={contacts} identity={identity} data={data} />
        );
      case "bill":
        return <Bill identity={identity} data={data} />;
      case "setting":
        return <SettingPage />;
      default:
        return <HomePage />;
    }
  };
  //identity if this empty
  return <>{activePage()}</>;
}
