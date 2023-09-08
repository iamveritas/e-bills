import React from "react";
import IconHolder from "../elements/IconHolder";
import ham from "../../assests/hamburger.svg";
import profile from "../../assests/profile.svg";
import back from "../../assests/backArrow.svg";
import SecondaryIcon from "../elements/SecondaryIcon";

export default function Header({ backHeader, title }) {
  if (backHeader) {
    return (
      <div className="header">
        <SecondaryIcon margin iconImage={back} />
        <span className="header-text">{title}</span>
        <IconHolder icon={profile} />
      </div>
    );
  } else {
    return (
      <div className="header">
        <IconHolder icon={ham} />
        <span className="header-text">{title}</span>
        <IconHolder icon={profile} />
      </div>
    );
  }
}
