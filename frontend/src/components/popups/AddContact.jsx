import React, { useState } from "react";
import { useContext } from "react";
import closeIcon from "../../assests/close-btn.svg";
import addIcon from "../../assests/add.svg";
import { MainContext } from "../../context/MainContext";

export default function AddContact({ handleAddContact }) {
  const { showPopUp } = useContext(MainContext);
  const [contact, setContact] = useState({ name: "", nodeId: "" });
  const handleChange = (e) => {
    setContact({ ...contact, [e.target.name]: e.target.value });
  };
  const handleSubmit = () => {
    handleAddContact(contact);
    showPopUp(false, "");
  };
  return (
    <div className="contact add-contact">
      <div className="contact-head">
        <span className="contact-head-title">CONTACTS</span>
        <img onClick={() => showPopUp(false, "")} src={closeIcon} />
      </div>
      <div className="contact-body">
        <input
          type="text"
          name="name"
          id="name"
          placeholder="Full Name"
          onChange={handleChange}
        />
        <input
          type="text"
          name="nodeId"
          id="nodeId"
          placeholder="Node Identity"
          onChange={handleChange}
        />
      </div>
      <button onClick={handleSubmit} className="btn">
        <img src={addIcon} />
        <span>ADD CONTACT</span>
      </button>
    </div>
  );
}
