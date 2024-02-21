import React, {useContext, useState} from "react";
import Header from "../sections/Header";
import TopDownHeading from "../elements/TopDownHeading";
import IconHolder from "../elements/IconHolder";
import attachment from "../../assests/attachment.svg";
import UniqueNumber from "../sections/UniqueNumber";
import {MainContext} from "../../context/MainContext";
import SelectSearchOption from "../elements/SelectSearchOption";

export default function SellPage({ data }) {
    const { handlePage, contacts, showPopUp, showPopUpSecondary, handleRefresh } =
        useContext(MainContext);

    const [dataForm, setDataForm] = useState({
        amount_numbers: "",
        billBuyer: "",
        currency_code: "sat",
    });

    const changeHandle = (e) => {
        let value = e.target.value;
        let name = e.target.name;
        setDataForm({ ...dataForm, [name]: value });
    };

    const handleSubmit = async () => {
        const form_data = new FormData();
        form_data.append("bill_name", data.name);
        form_data.append("buyer", dataForm.billBuyer);
        form_data.append("amount_numbers", dataForm.amount_numbers);

        await fetch("http://localhost:8000/bill/sell", {
            method: "POST",
            body: form_data,
            mode: "cors",
        })
            .then((response) => {
                console.log(response);
                showPopUpSecondary(false, "");
                showPopUp(false, "");
                handlePage("home");
                handleRefresh();
            })
            .catch((err) => err);
    };

    const checkHandleSearch = (e) => {
        let value = e.target.value;
        let name = e.target.name;
        const isValidOption = contacts.some((d) => d.name == value);
        if (isValidOption || value === "") {
            setDataForm(e.target.value);
        } else {
            setDataForm("");
        }
    };

    return (
        <div className="accept">
            <Header title="Sell" />
            <UniqueNumber UID={data.place_of_payment} date={data.date_of_issue} />
            <div className="head">
                <TopDownHeading upper="Against this" lower="Bill Of Exchange" />
                <IconHolder icon={attachment} />
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
                            <span className="accept-heading">sell to the order of </span>
                            <span className="block detail input-blank search-select">
                                <SelectSearchOption
                                    identity="drawee_name"
                                    placingHolder="Select Your Buyer"
                                    class="endorse-select"
                                    value={data.billBuyer}
                                    changeHandle={changeHandle}
                                    checkHandleSearch={checkHandleSearch}
                                    options={contacts}
                                />
                            </span>
                        </span>

                        <span className="block mt">
                            <label htmlFor="amount_numbers">the sum of</label>
                            <div className="form-input-row">
                                <span className="select-opt">
                                    <select
                                        style={{
                                            appearance: "none",
                                            MozAppearance: "none",
                                            WebkitAppearance: "none",
                                            textTransform: "uppercase",
                                        }}
                                        className="form-select"
                                        id="currency_code"
                                        name="currency_code"
                                        onChange={changeHandle}
                                        placeholder="sat"
                                        required
                                    >
                                        <option value={data.currency_code}>sat</option>
                                    </select>
                                </span>
                                <input
                                    className="drop-shadow"
                                    name="amount_numbers"
                                    value={data.amount_numbers}
                                    onChange={changeHandle}
                                    type="number"
                                    placeholder="10000"
                                    required
                                />
                            </div>
                    </span>


                    <span className="block mt">
                            <span className="accept-heading">Payer: </span>
                            <span className="block detail">
                                {data.drawee.name}, {data.place_of_drawing}
                            </span>
                        </span>
                        <span className="block mt">
                            <span className="accept-heading">Sold by: </span>
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
