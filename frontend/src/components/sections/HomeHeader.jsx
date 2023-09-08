import React from "react";
import IconHolder from "../elements/IconHolder";
import SecondaryIcon from "../elements/SecondaryIcon";

import setting from "../../assests/setting.svg";
import hamburger from "../../assests/hamburger.svg";
import payment_channel from "../../assests/payment-chanel.svg";
import payment_options from "../../assests/payment-options.svg";

export default function HomeHeader() {
    return (
        <div className="home-header">
            <div className="home-header-left">
                <IconHolder icon={setting}/>
                <IconHolder icon={hamburger}/>
            </div>
            <div className="home-header-right">
                <SecondaryIcon iconImage={payment_channel}/>
                <SecondaryIcon iconImage={payment_options}/>
            </div>
        </div>
    );
}
