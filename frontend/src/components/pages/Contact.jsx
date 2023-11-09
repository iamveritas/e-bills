import React, { useContext, useState } from "react";
import closeIcon from "../../assests/close-btn.svg";
import deleteBtn from "../../assests/delete.svg";
import { MainContext } from "../../context/MainContext";
import AddContact from "../popups/AddContact";
export default function Contact() {
  const { showPopUp, handlePage } = useContext(MainContext);
  let contactRay = [
    {
      name: "Mehfooz Salik",
      nodeId: "12D3KooWK9GBFBtuJMmF5RkEKznhggw17mJqMtA2nMfxbY1fME",
    },
    {
      name: "Mykyta Timko",
      nodeId: "12D3KooWK9GBFBtuJMmW2RkEKznhggw17mJqMtA2nMfxbY1fME",
    },
    {
      name: "Hubertus",
      nodeId: "12KooWK9GBFBtuJMmF5W2RkEKznhggw17mJqMtA2nMfxbY1fME",
    },
  ];
  const [contacts, setContacts] = useState(contactRay);
  const handleDelete = (id) => {
    let filtered = contacts.filter((d) => d.nodeId != id);
    setContacts(filtered);
  };
  const handleAddContact = (newContact) => {
    setContacts((prev) => [...prev, newContact]);
  };
  return (
    <div className="contact">
      <div className="contact-head">
        <span className="contact-head-title">CONTACTS</span>
        <img
          className="close-btn"
          onClick={() => {
            handlePage("home");
          }}
          src={closeIcon}
        />
      </div>
      <div className="contact-body">
        {contacts.map((d, i) => {
          return (
            <div key={i} className="contact-body-user">
              <span>{d.name}</span>
              <img onClick={() => handleDelete(d.nodeId)} src={deleteBtn} />
            </div>
          );
        })}
      </div>
      <button
        onClick={() =>
          showPopUp(true, <AddContact handleAddContact={handleAddContact} />)
        }
        className="btn"
      >
        ADD CONTACT
      </button>
    </div>
  );
}
