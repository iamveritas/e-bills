import React, {useContext, useEffect, useState} from "react";

import IssuePage from "./components/pages/IssuePage";
import AcceptPage from "./components/pages/AcceptPage";
import RepayPage from "./components/pages/RepayPage";
import Bill from "./components/pages/Bill";
import {MainContext} from "./context/MainContext";
import HomePage from "./components/pages/HomePage";
import ActivityPage from "./components/pages/ActivityPage";
import EndorsePage from "./components/pages/EndorsePage";
import ReqPaymentPage from "./components/pages/ReqPaymentPage";
import ReqAcceptPage from "./components/pages/ReqAcceptPage";
import IdentityPage from "./components/pages/IdentityPage";
import SettingPage from "./components/pages/SettingPage";
import ErrrorPage from "./components/pages/ErrrorPage";
import Contact from "./components/pages/Contact";

export default function App() {
    const {toast, loading, current, popUp, popUp2, contacts} =
        useContext(MainContext);
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

    const [operation_codes, setOperationCodes] = useState([]);
    // Set operation codes
    useEffect(() => {
        fetch("http://localhost:8000/opcodes/return")
            .then((res) => res.json())
            .then((data) => {
                setOperationCodes(data);
            })
            .catch((err) => {
                console.log(err.message);
            });
    }, []);

    const changeHandle = (e) => {
        let value = e.target.value;
        let name = e.target.name;
        setData({...data, [name]: value});
    };
    const checkHandleSearch = (e) => {
        let value = e.target.value;
        let name = e.target.name;
        const isValidOption = contacts.some((d) => d.name == value);
        if (isValidOption || value === "") {
            setData({...data, [name]: value});
        } else {
            setData({...data, [name]: ""});
        }
    };
    const handleChangeDrawerIsPayee = (e) => {
        let value = !data.drawer_is_payee;
        let name = e.target.name;
        setData({...data, [name]: value});
    };
    const handleChangeDrawerIsDrawee = (e) => {
        let value = !data.drawer_is_drawee;
        let name = e.target.name;
        setData({...data, [name]: value});
    };

    const activePage = () => {
        switch (current) {
            case "identity":
                return <IdentityPage/>;
            case "home":
                return <HomePage/>;
            case "activity":
                return <ActivityPage/>;
            case "accept":
                return <AcceptPage data={data}/>;
            case "reqaccept":
                return <ReqAcceptPage/>;
            case "reqpayment":
                return <ReqPaymentPage/>;
            case "endorse":
                return <EndorsePage/>;
            case "repay":
                return <RepayPage data={data}/>;
            case "contact":
                return <Contact/>;
            case "issue":
                return (
                    <IssuePage
                        data={data}
                        changeHandle={changeHandle}
                        checkHandleSearch={checkHandleSearch}
                        handleChangeDrawerIsDrawee={handleChangeDrawerIsDrawee}
                        handleChangeDrawerIsPayee={handleChangeDrawerIsPayee}
                    />
                );
            case "bill":
                return <Bill data={data}/>;
            case "setting":
                return <SettingPage/>;
            default:
                return <HomePage/>;
        }
    };
    //identity if this empty
    if (loading) {
        return (
            <div className="loading-main">
                <div className="loading">
                    <div className="loading-sub">
                        <div></div>
                    </div>
                </div>
            </div>
        );
    } else {
        return (
            <>
                {toast && <span className="toast">{toast}</span>}
                {popUp.show && <div className="popup">{popUp.content}</div>}
                {popUp2.show && <div className="popup">{popUp2.content}</div>}
                {activePage()}
            </>
        );
    }
}
