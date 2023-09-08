import React, {useContext} from "react";
import Header from "../sections/Header";
import TopDownHeading from "../elements/TopDownHeading";
import IconHolder from "../elements/IconHolder";
import attachment from "../../assests/attachment.svg";
import {MainContext} from "../../context/MainContext";

export default function RepayPage({contacts, identity, data}) {
    const {handlePage} = useContext(MainContext);
    return (
        <div className="Repay">
            <Header title="Pay"/>
            {/*<UniqueNumber UID="001" date="16-Feb-2023" />*/}
            <div className="head">
                <TopDownHeading upper="Against this" lower="Bill Of Exchange"/>
                <IconHolder icon={attachment}/>
            </div>
            <div className="col">
                <span>Send to</span>
                <span className="colored">{data.payee_name}</span>
            </div>
            <div className="inline">
                <span>the sum of </span>
                <span className="colored">
                    {data.currency_code} {data.amount_numbers}
                </span>
            </div>
            <div className="col mtt">
                <label htmlFor="wallet">Send from wallet:</label>
                <span className="select-opt">
                    <select name="wallet" id="wallet" required>
                        <option value="BTC" defaultValue="btc">
                            Default BTC wallet
                        </option>
                    </select>
                </span>
                <button className="btn mtt" onClick={() => handlePage("bill")}>
                    SIGN
                </button>
            </div>
        </div>
    );
}
