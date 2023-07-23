import React from "react";

export default function IssueForm({contacts, data, identity, changeHandle, handlePage}) {

    const handleSubmition = e => {
        e.preventDefault();

        const form_data = new FormData(e.target);

        fetch('http://localhost:8000/bill/issue', {
            method: 'POST',
            body: form_data,
            mode: 'no-cors',
        }).then(response => {
            console.log(response);
            handlePage("accept");
        }).catch(err => err);
    };

    let listContacts = contacts.map( contact => {
            return (
                <option key={contact.name}>{contact.name}</option>
            )
    })

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
            <div className="form-input">
                <label htmlFor="drawee_name">to the order of</label>
                <div className="form-input-row">
                    <select
                        id="payee_name"
                        name="payee_name"
                        value={data.payee_name}
                        onChange={changeHandle}
                        placeholder="Payee Company, Zurich"
                        required
                    >
                        {listContacts}
                    </select>
                </div>
            </div>
            <div className="form-input">
                <label htmlFor="amount_numbers">the sum of</label>
                <div className="form-input-row">
                    <span className="select-opt">
                        <select
                            className="form-select"
                            id="currency_code"
                            name="currency_code"
                            onChange={changeHandle}
                            placeholder="EUR"
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
            <div className="form-input">
                <label htmlFor="drawee_name">Drawee</label>
                <div className="form-input-row">
                    <select
                        id="drawee_name"
                        name="drawee_name"
                        placeholder="Drawee Company, Vienna"
                        value={data.drawee_name}
                        onChange={changeHandle}
                        required
                    >
                        {listContacts}
                    </select>
                </div>
            </div>
            <div className="form-input">
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
            <div className="form-input">
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
            <input className="btn" type="submit" value="Issue bill"/>
        </form>
    );
}
