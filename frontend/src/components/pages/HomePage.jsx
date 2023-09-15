import React from "react";
import HomeHeader from "../sections/HomeHeader";
import Amount from "../elements/Amount";
import SubHeading from "../elements/SubHeading";
import SubTotal from "../elements/SubTotal";
import BillDetails from "../elements/BillDetails";
import Payments from "../elements/Payments";
import bills from "../../assests/bills.svg";

export default function HomePage({bills_list}) {
    return (
        <div className="home">
            <HomeHeader/>
            <div className="home-container">
                <div className="home-container-amount">
                    <Amount color="38A665" currency="BTC" amount="1.021" degree="0"/>
                    <hr className="home-container-amount-line"/>
                    <Amount color="C30000" currency="BTC" amount="1.011" degree="180"/>
                </div>
                <div className="home-container-heading">
                    <SubHeading rotate="0" route="issue" color="38A665" currency="Bill"/>
                    <SubHeading rotate="180" route="dont" color="C30000" currency="IOU"/>
                </div>
                <div className="home-container-total">
                    <SubTotal color="a3a3a3" currency="BTC" amount="7.01"/>
                </div>
                <div className="line"></div>
                <div className="home-container-bills">
                    <BillDetails
                        color="a3a3a3"
                        data={bills_list?.slice(0, 4)}
                        icon={bills}
                    />
                </div>
                {bills_list?.length > 4 && (
                    <div className="home-container-payments">
                        <Payments
                            payments="4 Recent Payments"
                            history="Full Payment History"
                        />
                    </div>
                )}
            </div>
        </div>
    );
}
