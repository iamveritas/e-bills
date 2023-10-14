import React, { useContext } from "react";
import IconHolder from "../elements/IconHolder";
import SecondaryIcon from "../elements/SecondaryIcon";

import setting from "../../assests/setting.svg";
import hamburger from "../../assests/hamburger.svg";
import payment_options from "../../assests/payment-options.svg";
import payment_channel from "../../assests/payment-chanel.svg";
import profile from "../../assests/profile.svg";
import { MainContext } from "../../context/MainContext";

export default function HomeHeader() {
  const { handlePage } = useContext(MainContext);
  return (
    <div className="home-header">
      <div className="home-header-left">
        <IconHolder handleClick={() => handlePage("setting")} icon={setting} />
        <IconHolder handleClick={() => handlePage("dont")} icon={hamburger} />
      </div>
      <div className="home-header-right">
        <SecondaryIcon routing="dont" iconImage={payment_channel} />
        <IconHolder handleClick={() => handlePage("identity")} icon={profile} />
      </div>
    </div>
  );
}
