import React, {useContext, useState} from "react";
import Header from "../sections/Header";
import TopDownHeading from "../elements/TopDownHeading";
import IconHolder from "../elements/IconHolder";
import attachment from "../../assests/attachment.svg";
import UniqueNumber from "../sections/UniqueNumber";
import {MainContext} from "../../context/MainContext";

export default function EndorsePage({data}) {
    const {handlePage, contacts, showPopUp} = useContext(MainContext);

    const [billEndorse, setBillEndorse] = useState(contacts[0].name);
    const changeHandle = (e) => {
        setBillEndorse(e.target.value);
    };
    console.log(data.name, billEndorse);
    const handleSubmit = async () => {
        const form_data = new FormData();
        form_data.append("bill_name", data.name);
        form_data.append("endorsee", billEndorse);
        await fetch("http://localhost:8000/bill/endorse", {
            method: "POST",
            body: form_data,
            mode: "no-cors",
        })
            .then((response) => {
                console.log(response);
                showPopUp(false, "");
                handlePage("home");
            })
            .catch((err) => err);
    };
    return (
        <div className="accept">
            <Header title="Endorse"/>
            <UniqueNumber UID={data.place_of_payment} date={data.date_of_issue}/>
            <div className="head">
                <TopDownHeading upper="Against this" lower="Bill Of Exchange"/>
                <IconHolder icon={attachment}/>
            </div>
            <div className="accept-container">
                <div className="accept-container-content">
                    <div className="block mt">
                        <span className="block">
                            <span className="accept-heading">pay on </span>
                            <span className="detail">{data.date_of_issue}</span>
                        </span>
                        <span className="block">
                            <span className="accept-heading">the sum of </span>
                            <span className="detail" style={{textTransform: "uppercase"}}>
                                {data.currency_code} {data.amount_numbers}
                            </span>
                        </span>
                        <span className="block mt">
                            <span className="accept-heading">to the order of </span>
                            <span className="block detail input-blank">
                                <select
                                    style={{appereance: "none"}}
                                    id="drawee_name"
                                    className="endorse-select"
                                    name="drawee_name"
                                    placeholder="Drawee Company, Vienna"
                                    value={billEndorse}
                                    onChange={changeHandle}
                                >
                                    {contacts.map((d) => (
                                        <option value={d.name}>{d.name}</option>
                                    ))}
                                </select>
                            </span>
                        </span>
                        <span className="block mt">
                            <span className="accept-heading">Payer: </span>
                            <span className="block detail">
                                {data.drawee.name}, {data.place_of_drawing}
                            </span>
                        </span>
                        <span className="block mt">
                            <span className="accept-heading">Endorsed by: </span>
                            <span className="block detail">
                                {data.payee.name}, {data.place_of_payment}
                            </span>
                        </span>
                    </div>
                    <button className="btn mtt" onClick={handleSubmit}>
                        SIGN
                    </button>
                </div>
            </div>
        </div>
    );
}
