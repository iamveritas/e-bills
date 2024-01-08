import React, { useState } from "react";
import deleteBtn from "../assests/delete.svg";
export default function SingleContact({ handleDelete, name, PeerId }) {
  const [displayName, setDisplayName] = useState(name);
  const handleName = () => {
    if (displayName === name) {
      setDisplayName(PeerId);
    } else {
      setDisplayName(name);
    }
  };
  return (
    <div className="contact-body-user">
      <span onClick={handleName}>{displayName}</span>
      <img onClick={() => handleDelete(name)} src={deleteBtn} />
    </div>
  );
}
