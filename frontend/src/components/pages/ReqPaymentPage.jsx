import React, {useContext} from "react";
import Header from "../sections/Header";
import TopDownHeading from "../elements/TopDownHeading";
import IconHolder from "../elements/IconHolder";
import attachment from "../../assests/attachment.svg";
import UniqueNumber from "../sections/UniqueNumber";
import {MainContext} from "../../context/MainContext";

export default function ReqPaymentPage() {
    const {handlePage} = useContext(MainContext);
    return (
        <div className="accept">
            <Header title="Request Payment"/>
            <UniqueNumber UID="NY" date="16-Feb-2023"/>
            <div className="head">
                <TopDownHeading upper="Against this" lower="Bill Of Exchange"/>
                <IconHolder icon={attachment}/>
            </div>
            <div className="accept-container">
                <div className="accept-container-content">
                    <div className="block mt">
                        <span className="block mt">
                            <span className="accept-heading">send to </span>
                            <span className="block detail input-blank">
                                Payee Company, NY
                            </span>
                        </span>
                        <span className="block mt">
                            <span className="accept-heading">the sum of </span>
                            <span className="detail">BTC 3.125</span>
                        </span>
                        <span className="block mtt">
                            <span className="accept-heading">Requested by: </span>
                            <span className="block detail">Payee Company, NY</span>
                        </span>
                    </div>
                    <button className="btn mtt" onClick={() => handlePage("repay")}>
                        SIGN
                    </button>
                </div>
            </div>
        </div>
    );
}
