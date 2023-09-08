import React from "react";
import Arrow from "./Arrow";

function SubHeading({color, amount, currency}) {
    return (
        <div className="home-container-heading-single ">
            <div className="arrow">
                <Arrow color={color}/>
            </div>
            <span style={{color: `#${color}`}}>
                {amount} {currency}
            </span>
        </div>
    );
}

export default SubHeading;
