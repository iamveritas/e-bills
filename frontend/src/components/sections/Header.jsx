import React from "react";
import IconHolder from "../elements/IconHolder";

import ham from "../../assests/hamburger.svg";
import profile from "../../assests/profile.svg";

export default function Header({ title }) {
  return (
    <div className="header">
      <IconHolder icon={ham} />
      <span className="header-text">{title}</span>
      <IconHolder icon={profile} />
    </div>
  );
}
