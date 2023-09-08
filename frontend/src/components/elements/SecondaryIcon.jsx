import React from "react";

export default function SecondaryIcon({iconImage, margin}) {
    return (
        <div className="secondary-icon">
            <img
                style={{marginRight: `${margin ? "0.5vw" : "0"}`}}
                className="secondary-icon-image"
                src={iconImage}
            />
        </div>
    );
}
