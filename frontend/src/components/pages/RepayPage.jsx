import React from "react";

export default function RepayPage({identity, data, handlePage}) {
    return (
        <div className="Repay">
            <div className="col">
                <span>Send to</span>
                <span className="colored">{data.payee_name}</span>
            </div>
            <div className="inline">
                <span>the sum of </span>
                <span className="colored">
                    {data.currency_code} {data.amount_numbers}
                </span>
            </div>
            <div className="col mtt">
                <label htmlFor="wallet">Send from wallet:</label>
                <span className="select-opt">
                    <select name="wallet" id="wallet" required>
                    <option value="BTC" defaultValue="btc">
                        Default BTC wallet
                    </option>
                    </select>
                </span>
                <button className="btn mtt" onClick={() => handlePage("bill")}>
                    SIGN
                </button>
            </div>
        </div>
    );
}
