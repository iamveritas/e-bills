import React from "react";

export default function IssueForm({
  contacts,
  data,
  identity,
  changeHandle,
  handlePage,
  handleChangeDrawerIsDrawee,
  handleChangeDrawerIsPayee,
}) {
  const handleSubmition = (e) => {
    e.preventDefault();

    const form_data = new FormData(e.target);
    fetch("http://localhost:8000/bill/issue", {
      method: "POST",
      body: form_data,
      mode: "no-cors",
    })
      .then((response) => {
        console.log(response);
        handlePage("bill");
      })
      .catch((err) => err);
  };

  let listContacts = contacts.map((contact) => {
    return <option key={contact.name}>{contact.name}</option>;
  });

  return (
    <form className="form" onSubmit={handleSubmition}>
      <div className="form-input">
        <label htmlFor="maturity_date">Maturity date</label>
        <div className="form-input-row">
          <input
            className="drop-shadow"
            id="maturity_date"
            name="maturity_date"
            value={data.maturity_date}
            onChange={changeHandle}
            type="date"
            placeholder="16 May 2023"
            required
          />
        </div>
      </div>
      <div className="flex-row">
        <div className="form-input flex-grow">
          <label htmlFor="drawee_name">to the order of</label>
          <div className="form-input-row">
            <select
              className="select-class"
              disabled={data.drawer_is_payee || data.drawee_name}
              style={{ appereance: "none" }}
              id="payee_name"
              name="payee_name"
              value={data.payee_name}
              onChange={changeHandle}
              placeholder="Payee Company, Zurich"
            >
              <option value=""></option>
              {listContacts}
            </select>
          </div>
        </div>
        <label className="flex-col align-center" htmlFor="drawer_is_payee">
          <span>ME</span>
          <div className="form-input-row">
            <input
              disabled={data.drawer_is_drawee || data.payee_name}
              type="checkbox"
              id="drawer_is_payee"
              name="drawer_is_payee"
              checked={data.drawer_is_payee}
              onChange={handleChangeDrawerIsPayee}
            />
            <svg
              className="check-boxes"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                id="checkbox"
                d="M31.6757 19.7629C32.0828 20.136 32.1103 20.7686 31.7372 21.1757L23.4872 30.1757C23.2977 30.3824 23.0303 30.5 22.75 30.5C22.4697 30.5 22.2023 30.3824 22.0128 30.1757L18.2628 26.0848C17.8897 25.6777 17.9172 25.0451 18.3243 24.6719C18.7314 24.2988 19.364 24.3263 19.7372 24.7334L22.75 28.0201L30.2628 19.8243C30.636 19.4172 31.2686 19.3897 31.6757 19.7629Z"
                fill={`#${data.drawer_is_payee ? "F7931A" : "545454"}`}
              />
              <rect
                x="1vw"
                y="1vw"
                id="checkbox"
                rx="1vw"
                stroke={`#${data.drawer_is_payee ? "F7931A" : "545454"}`}
                stroke-width="1vw"
              />
            </svg>
          </div>
        </label>
      </div>
      <div className="form-input">
        <label htmlFor="amount_numbers">the sum of</label>
        <div className="form-input-row">
          <span className="select-opt">
            <select
              style={{
                appereance: "none",
                MozAppearance: "none",
                WebkitAppearance: "none",
                textTransform: "uppercase",
              }}
              className="form-select"
              id="currency_code"
              name="currency_code"
              onChange={changeHandle}
              placeholder="SATS"
              required
            >
              <option value={data.currency_code}>sats</option>
            </select>
          </span>
          <input
            className="drop-shadow"
            name="amount_numbers"
            value={data.amount_numbers}
            onChange={changeHandle}
            type="number"
            placeholder="10000"
            required
          />
        </div>
      </div>
      <div className="flex-row">
        <div className="form-input flex-grow">
          <label htmlFor="drawee_name">Drawee</label>
          <div className="form-input-row">
            <select
              disabled={data.drawer_is_drawee || data.payee_name}
              style={{
                appereance: "none",
                MozAppearance: "none",
                WebkitAppearance: "none",
              }}
              id="drawee_name"
              name="drawee_name"
              placeholder="Drawee Company, Vienna"
              value={data.drawee_name}
              onChange={changeHandle}
            >
              <option value=""></option>
              {listContacts}
            </select>
          </div>
        </div>
        <label className="flex-col align-center" htmlFor="drawer_is_drawee">
          <span>ME</span>
          <div className="form-input-row">
            <input
              disabled={data.drawer_is_payee || data.drawee_name}
              type="checkbox"
              id="drawer_is_drawee"
              name="drawer_is_drawee"
              onChange={handleChangeDrawerIsDrawee}
              checked={data.drawer_is_drawee}
            />
            <svg
              className="check-boxes"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                id="checkbox"
                d="M31.6757 19.7629C32.0828 20.136 32.1103 20.7686 31.7372 21.1757L23.4872 30.1757C23.2977 30.3824 23.0303 30.5 22.75 30.5C22.4697 30.5 22.2023 30.3824 22.0128 30.1757L18.2628 26.0848C17.8897 25.6777 17.9172 25.0451 18.3243 24.6719C18.7314 24.2988 19.364 24.3263 19.7372 24.7334L22.75 28.0201L30.2628 19.8243C30.636 19.4172 31.2686 19.3897 31.6757 19.7629Z"
                fill={`#${data.drawer_is_drawee ? "F7931A" : "545454"}`}
              />
              <rect
                x="1vw"
                y="1vw"
                id="checkbox"
                rx="1vw"
                stroke={`#${data.drawer_is_drawee ? "F7931A" : "545454"}`}
                stroke-width="1vw"
              />
            </svg>
          </div>
        </label>
      </div>
      <div className="form-input" hidden={true}>
        <label htmlFor="drawer_name">Drawer</label>
        <div className="form-input-row">
          <input
            id="drawer_name"
            name="drawer_name"
            value={identity.name}
            onChange={changeHandle}
            type="text"
            placeholder="Drawer Company, London"
            readOnly={true}
            required
          />
        </div>
      </div>
      <div className="form-input">
        <label htmlFor="place_of_drawing">Place of drawing</label>
        <div className="form-input-row">
          <input
            id="place_of_drawing"
            name="place_of_drawing"
            value={data.place_of_drawing}
            onChange={changeHandle}
            type="text"
            placeholder="Zurich"
            required
          />
        </div>
      </div>
      <div className="form-input">
        <label htmlFor="place_of_payment">Place of payment</label>
        <div className="form-input-row">
          <input
            id="place_of_payment"
            name="place_of_payment"
            value={data.place_of_payment}
            onChange={changeHandle}
            type="text"
            placeholder="London"
            required
          />
        </div>
      </div>
      <div className="form-input">
        <label htmlFor="bill_jurisdiction">Bill jurisdiction</label>
        <div className="form-input-row">
          <input
            id="bill_jurisdiction"
            name="bill_jurisdiction"
            value={data.bill_jurisdiction}
            onChange={changeHandle}
            type="text"
            placeholder="UK"
            required
          />
        </div>
      </div>
      <div className="form-input" hidden={true}>
        <label htmlFor="language">Language</label>
        <div className="form-input-row">
          <input
            id="language"
            name="language"
            value={data.language}
            onChange={changeHandle}
            type="text"
            required
            readOnly={true}
          />
        </div>
      </div>
      <div className="form-input">
        <label htmlFor="maturity_date">Date of issue</label>
        <div className="form-input-row">
          <input
            className="drop-shadow"
            id="date_of_issue"
            name="date_of_issue"
            value={data.date_of_issue}
            onChange={changeHandle}
            type="date"
            placeholder="16 May 2023"
            required
          />
        </div>
      </div>
      <input className="btn" type="submit" value="Issue bill" />
    </form>
  );
}
