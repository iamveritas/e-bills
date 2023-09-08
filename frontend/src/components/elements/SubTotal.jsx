import React from "react";
import Arrow from "./Arrow";

function SubTotal({ color, amount, currency }) {
  return (
    <div className="home-container-total-single ">
      <Arrow color={color} />
      <span style={{ color: `#${color}` }}>
        {amount} {currency}
      </span>
    </div>
  );
}

export default SubTotal;
