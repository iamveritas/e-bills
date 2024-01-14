import React, { useState } from "react";
import deleteBtn from "../assests/delete.svg";
import editBtn from "../assests/edit.svg";
import copyIcon from "../assests/copy.svg";
import EditContact from "./popups/EditContact";
export default function SingleContact({
  copytoClip,
  showPopUp,
  handleDelete,
  name,
  PeerId,
}) {
  const [displayName, setDisplayName] = useState(name);
  const handleName = () => {
    if (displayName === name) {
      setDisplayName(
        PeerId.slice(0, 5) +
          "..." +
          PeerId.slice(PeerId.length - 5, PeerId.length)
      );
    } else {
      setDisplayName(name);
    }
  };
  const copyPeerId = () => {
    if (displayName !== name)
      copytoClip(PeerId, `${name} Node Identity is copied`);
  };
  return (
    <div className="contact-body-user">
      <span className="flexgap" onClick={copyPeerId}>
        <span className="contact-body-user-name" onClick={handleName}>
          {displayName}
        </span>
        {displayName !== name && <img className="copyicon" src={copyIcon} />}
      </span>
      <span className="flexgap">
        <img
          onClick={() =>
            showPopUp(true, <EditContact old_name={displayName} />)
          }
          src={editBtn}
        />
        <img onClick={() => handleDelete(name)} src={deleteBtn} />
      </span>
    </div>
  );
}
