import React, {useContext} from "react";
import IconHolder from "../elements/IconHolder";
import ham from "../../assests/hamburger.svg";
import profile from "../../assests/profile.svg";
import cross from "../../assests/cross.svg";
import back from "../../assests/backArrow.svg";
import SecondaryIcon from "../elements/SecondaryIcon";
import {MainContext} from "../../context/MainContext";

export default function Header({route, backHeader, title}) {
    const {showPopUpSecondary, handlePage} = useContext(MainContext);
    if (backHeader) {
        return (
            <div className="header">
                <SecondaryIcon
                    routing={() => handlePage(route)}
                    margin
                    iconImage={back}
                />
                <span className="header-text">{title}</span>
                <IconHolder handleClick={() => handlePage("identity")} icon={profile}/>
            </div>
        );
    } else {
        return (
            <div className="header">
                <span className="header-text">{title}</span>
                <SecondaryIcon
                    routing={
                        title === "Issue"
                            ? () => handlePage("home")
                            : () => showPopUpSecondary(false, "")
                    }
                    margin
                    iconImage={cross}
                />
            </div>
        );
    }
}
