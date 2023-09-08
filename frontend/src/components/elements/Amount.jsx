import React from "react";
import Arrow from "../elements/Arrow";

export default function Amount({color, amount, currency, degree}) {
    return (
        <div className="home-container-amount-single ">
            <Arrow color={color} degree={degree}/>
            <span style={{color: `#${color}`}}>
                {amount} {currency}
            </span>
        </div>
    );
}
