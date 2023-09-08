import React from "react";

function Payments({payments, history}) {
    return (
        <div className="home-container-payments-details">
            <div className="payments">{payments}</div>
            <div className="history">{history}</div>
        </div>
    );
}

export default Payments;
