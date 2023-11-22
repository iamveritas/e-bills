import React, {useContext, useState} from "react";
import closeIcon from "../../assests/close-btn.svg";
import deleteBtn from "../../assests/delete.svg";
import {MainContext} from "../../context/MainContext";
import AddContact from "../popups/AddContact";

export default function Contact() {
    const {showPopUp, handlePage, handleDelete, contacts} =
        useContext(MainContext);
    const [search, setSearch] = useState("");
    let newContact;
    if (search) {
        newContact = contacts.filter((d) => d.name.includes(search));
    } else {
        newContact = contacts;
    }
    const handleSearchChange = (e) => {
        setSearch(e.target.value);
    };
    return (
        <div className="contact">
            <div className="contact-head">
                <span className="contact-head-title">CONTACT</span>
                <img
                    className="close-btn"
                    onClick={() => {
                        handlePage("home");
                    }}
                    src={closeIcon}
                />
            </div>
            <input
                type="text"
                className="input-contact"
                placeholder="Search Contact"
                onChange={handleSearchChange}
            />
            <div className="contact-body">
                {newContact.map((d, i) => {
                    return (
                        <div key={i} className="contact-body-user">
                            <span>{d.name}</span>
                            <img onClick={() => handleDelete(d.name)} src={deleteBtn}/>
                        </div>
                    );
                })}
            </div>
            <button onClick={() => showPopUp(true, <AddContact/>)} className="btn">
                ADD CONTACT
            </button>
        </div>
    );
}
