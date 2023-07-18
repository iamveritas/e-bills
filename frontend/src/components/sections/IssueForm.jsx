import React from "react";

export default function IssueForm({ data, changeHandle, handlePage }) {
  const handleSubmition = () => {
    handlePage("accept");
  };
  return (
    <form className="form" onSubmit={handleSubmition}>
      <div className="form-input">
        <label htmlFor="payon">Pay on</label>
        <div className="form-input-row">
          <span className="select-opt">
            <select
              className="form-select"
              id="payon"
              name="payon"
              value={data.payon}
              onChange={changeHandle}
              type="text"
              placeholder="3M"
              required
            >
              <option value="2M">2M</option>
              <option value="3M">3M</option>
              <option value="4M">4M</option>
            </select>
          </span>
          <input
            className="drop-shadow"
            id="payon-date"
            name="payonDate"
            value={data.payonDate}
            onChange={changeHandle}
            type="date"
            placeholder="16 May 2023"
            required
          />
        </div>
      </div>
      <div className="form-input">
        <label htmlFor="to">to the order of</label>
        <div className="form-input">
          <input
            id="to"
            name="toOrder"
            value={data.toOrder}
            onChange={changeHandle}
            type="text"
            placeholder="Beneficiary Company, NY"
            required
          />
        </div>
      </div>
      <div className="form-input">
        <label htmlFor="sum">the sum of</label>
        <div className="form-input-row">
          <span className="select-opt">
            <select
              className="form-select"
              id="sum"
              name="toSumCurrency"
              value={data.toSumCurrency}
              onChange={changeHandle}
              type="text"
              placeholder="EUR"
              required
            >
              <option value="EURO">Euro</option>
              <option value="DOLLAR">Dollar</option>
              <option value="PK Rupee">Pkr</option>
            </select>
          </span>
          <input
            className="drop-shadow"
            name="toSumAmount"
            value={data.toSumAmount}
            onChange={changeHandle}
            type="number"
            placeholder="3,125.00"
            required
          />
        </div>
      </div>
      <div className="form-input">
        <label htmlFor="drawe">Drawee</label>
        <div className="form-input">
          <input
            id="drawe"
            name="drawee"
            value={data.drawee}
            onChange={changeHandle}
            type="text"
            placeholder="Drawee Company, Vienna"
            required
          />
        </div>
      </div>
      <input className="btn" type="submit" value="SIGN" />
    </form>
  );
}
