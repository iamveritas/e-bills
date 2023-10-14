import React, { useContext, useEffect, useState } from "react";

import avatar from "../../assests/avatar.svg";
import closebtn from "../../assests/close-btn.svg";
import { MainContext } from "../../context/MainContext";

export default function IdentityPage({ identity }) {
  const { handlePage } = useContext(MainContext);
  const [userData, setuserData] = useState({
    name: identity.name || "",
    phone_number: identity.phone_number || "",
    email: identity.email || "",
    date_of_birth: identity.date_of_birth || "",
    country_of_birth: identity.country_of_birth || "",
    city_of_birth: identity.city_of_birth || "",
    postal_address: identity.postal_address || "",
  });
  const [image, setImage] = useState();
  const [uneditable, setunEditable] = useState(true);
  const [content, setContent] = useState({
    justify: "",
    close: false,
    sign: true,
  });
  const onChangeHandler = (e) => {
    let values = e.target.value;
    let name = e.target.name;
    setuserData({ ...userData, [name]: values });
  };
  const handleFileChange = (e) => {
    const file = e.target.files[0];

    if (file) {
      if (file.type === "image/jpeg" || file.name.endsWith(".jpg")) {
        setImage(URL.createObjectURL(file));
      } else {
        setImage(avatar);
      }
    }
  };

  useEffect(() => {
    if (identity.name && identity.email) {
      setContent({
        justify: " justify-space",
        close: true,
        sign: false,
      });
    } else {
      setunEditable(false);
    }
  }, []);

  return (
    <div className="create">
      <div className={"create-head" + content.justify}>
        {content.sign ? (
          <span className="create-head-title">Create Idenity</span>
        ) : (
          <span className="create-head-title">
            {!uneditable ? "Edit Idenity" : "Idenity"}
          </span>
        )}
        {content.close && (
          <img
            onClick={() => handlePage("home")}
            className="close-btn"
            src={closebtn}
          />
        )}
      </div>
      <div className="create-body">
        <div className="create-body-avatar">
          <input
            disabled={uneditable}
            onChange={handleFileChange}
            type="file"
            id="avatar"
          />
          <label htmlFor="avatar">
            <img src={image ? image : avatar} />
            <span>{image ? "Change Photo" : "Add Photo"}</span>
          </label>
        </div>
        <div className="create-body-form">
          <div className="create-body-form-input">
            <div className="create-body-form-input-in">
              <label htmlFor="name">Full Name</label>
              <input
                id="name"
                value={userData.name}
                disabled={uneditable}
                name="name"
                onChange={onChangeHandler}
                placeholder="Full Name"
                type="text"
              />
            </div>
            <div className="create-body-form-input-in">
              <label htmlFor="phonenumber">Phone Number</label>
              <input
                id="phonenumber"
                value={userData.phone_number}
                disabled={uneditable}
                name="phone_number"
                onChange={onChangeHandler}
                placeholder="Phone Number"
                type="text"
              />
            </div>
            <div className="create-body-form-input-in">
              <label htmlFor="emailaddress">Email Address</label>
              <input
                id="emailaddress"
                value={userData.email}
                disabled={uneditable}
                name="email"
                onChange={onChangeHandler}
                placeholder="Email Address"
                type="text"
              />
            </div>
            <div className="create-body-form-input-in">
              <label htmlFor="dob">Date Of Birth</label>
              <input
                id="dob"
                value={userData.date_of_birth}
                disabled={uneditable}
                name="date_of_birth"
                onChange={onChangeHandler}
                placeholder=""
                type="date"
              />
            </div>
          </div>
          <div className="create-body-form-input">
            <div className="create-body-form-input-in">
              <label htmlFor="countryofbirth">Country Of Birth</label>
              <input
                id="countryofbirth"
                value={userData.country_of_birth}
                disabled={uneditable}
                name="country_of_birth"
                onChange={onChangeHandler}
                placeholder="Country Of Birth"
                type="text"
              />
            </div>
            <div className="create-body-form-input-in">
              <label htmlFor="cityofbirth">City Of Birth</label>
              <input
                id="cityofbirth"
                value={userData.city_of_birth}
                disabled={uneditable}
                name="city_of_birth"
                onChange={onChangeHandler}
                placeholder="Country Of Birth"
                type="text"
              />
            </div>
            <div className="create-body-form-input-in">
              <label htmlFor="postal">Postal Address</label>
              <input
                id="postal"
                value={userData.postal_address}
                disabled={uneditable}
                name="postal_address"
                onChange={onChangeHandler}
                placeholder="Postal Address"
                type="text"
              />
            </div>
          </div>
        </div>
        {content.sign ? (
          <input className="create-body-btn" type="submit" value="SIGN" />
        ) : (
          <div className="flex justify-space">
            <button
              onClick={() => setunEditable(!uneditable)}
              className="create-body-btn"
            >
              {!uneditable ? "CANCEL" : "PREVIEW"}
            </button>
            {!uneditable && (
              <input
                className="create-body-btn"
                type="submit"
                value="SIGN CHANGES"
              />
            )}
          </div>
        )}
      </div>
    </div>
  );
}
