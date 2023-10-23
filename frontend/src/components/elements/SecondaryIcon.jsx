import React, {useContext} from "react";
import {MainContext} from "../../context/MainContext";

export default function SecondaryIcon({iconImage, margin, routing}) {
    return (
        <div className="secondary-icon" onClick={routing}>
            <img
                style={{marginRight: `${margin ? "0.5vw" : "0"}`}}
                className="secondary-icon-image"
                src={iconImage}
            />
        </div>
    );
}
