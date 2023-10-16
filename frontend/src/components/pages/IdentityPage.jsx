import React, { useContext, useEffect, useState } from "react";

import avatar from "../../assests/avatar.svg";
import closebtn from "../../assests/close-btn.svg";
import { MainContext } from "../../context/MainContext";

export default function IdentityPage({ identity }) {
  const { handlePage } = useContext(MainContext);
  const [userData, setuserData] = useState({
    name: identity.name || "",
    email: identity.email || "",
    date_of_birth:
      new Date(identity.date_of_birth).toLocaleDateString("en-CA") || "",
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

  const handleSubmit = async (e) => {
    e.preventDefault();
    const form_data = new FormData(e.target);
    // const form_data = JSON.stringify(userData);
    await fetch("http://localhost:8000/identity/create", {
      method: "POST",
      body: form_data,
      mode: "no-cors",
    })
      .then((response) => {
        console.log(response);
      })
      .catch((err) => err);
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
          <span className="create-head-title">Create Identity</span>
        ) : (
          <span className="create-head-title">
            {!uneditable ? "Edit Identity" : "Identity"}
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
      <form onSubmit={handleSubmit} className="create-body">
        <div className="create-body-avatar">
          {/* <input
            disabled={uneditable}
            onChange={handleFileChange}
            type="file"
            id="avatar"
          /> */}
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
                name="name"
                value={userData.name}
                disabled={uneditable}
                onChange={onChangeHandler}
                placeholder="Full Name"
                type="text"
              />
            </div>
            {/* <div className="create-body-form-input-in">
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
            </div> */}
            <div className="create-body-form-input-in">
              <label htmlFor="email">Email Address</label>
              <input
                id="email"
                name="email"
                value={userData.email}
                disabled={uneditable}
                onChange={onChangeHandler}
                placeholder="Email Address"
                type="text"
              />
            </div>
            <div className="create-body-form-input-in">
              <label htmlFor="date_of_birth">Date Of Birth</label>
              <input
                id="date_of_birth"
                name="date_of_birth"
                value={userData.date_of_birth}
                disabled={uneditable}
                onChange={onChangeHandler}
                placeholder=""
                type="date"
              />
            </div>
          </div>
          <div className="create-body-form-input">
            <div className="create-body-form-input-in">
              <label htmlFor="country_of_birth">Country Of Birth</label>
              <input
                id="country_of_birth"
                name="country_of_birth"
                value={userData.country_of_birth}
                disabled={uneditable}
                onChange={onChangeHandler}
                placeholder="Country Of Birth"
                type="text"
              />
            </div>
            <div className="create-body-form-input-in">
              <label htmlFor="city_of_birth">City Of Birth</label>
              <input
                id="city_of_birth"
                name="city_of_birth"
                value={userData.city_of_birth}
                disabled={uneditable}
                onChange={onChangeHandler}
                placeholder="Country Of Birth"
                type="text"
              />
            </div>
            <div className="create-body-form-input-in">
              <label htmlFor="postal_address">Postal Address</label>
              <input
                id="postal_address"
                name="postal_address"
                value={userData.postal_address}
                disabled={uneditable}
                onChange={onChangeHandler}
                placeholder="Postal Address"
                type="text"
              />
            </div>
          </div>
        </div>
        {content.sign && (
          <div className="flex justify-space">
            <div
              onClick={() => setunEditable(!uneditable)}
              className="create-body-btn"
            >
              {uneditable ? "CANCEL" : "PREVIEW"}
            </div>
            {uneditable && (
              <input className="create-body-btn" type="submit" value="SIGN" />
            )}
          </div>
        )}
      </form>
    </div>
  );
}
