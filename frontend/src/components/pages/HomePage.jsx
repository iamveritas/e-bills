import React from "react";
import HomeHeader from "../sections/HomeHeader";
import Amount from "../elements/Amount";
import SubHeading from "../elements/SubHeading";
import SubTotal from "../elements/SubTotal";
import BillDetails from "../elements/BillDetails";
import Payments from "../elements/Payments";
import bills from "../../assests/bills.svg";

export default function HomePage({
                                     bills_list,
                                 }) {
    return (
        <div className="home">
            <HomeHeader/>
            <div className="home-container">
                <div className="home-container-amount">
                    <Amount color="38A665" currency="BTC" amount="1.021" degree="0deg"/>
                    <hr className="home-container-amount-line"/>
                    <Amount color="C30000" currency="BTC" amount="1.011" degree="180deg"/>
                </div>
                <div className="home-container-heading">
                    <SubHeading color="38A665" currency="Bill"/>
                    <SubHeading color="C30000" currency="IOU"/>
                </div>
                <div className="home-container-total">
                    <SubTotal color="a3a3a3" currency="BTC" amount="7.01" degree="0deg"/>
                </div>
                <div className="line"></div>
                <div className="home-container-bills">
                    <BillDetails color="a3a3a3" data={bills_list} icon={bills}/>
                </div>
                <div className="home-container-payments">
                    <Payments payments="4 Recent Payments" history="Full Payment History"/>
                </div>
            </div>
        </div>
    );
}
