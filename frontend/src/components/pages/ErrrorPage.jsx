import React, { useContext } from "react";
import closeIcon from "../../assests/close-btn.svg";
import logo from "../../assests/logo.png";
import { MainContext } from "../../context/MainContext";

export default function ErrrorPage() {
  const { handlePage } = useContext(MainContext);
  return (
    <div className="error">
      <div className="error-head">
        <span className="error-head-title">
          <img src={logo} />
        </span>
        <svg
          className="close-btn"
          onClick={() => handlePage("home")}
          src={closeIcon}
          viewBox="0 0 46 46"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <rect
            x="0.1vw"
            y="0.1vw"
            width="9.9vw"
            height="9.9vw"
            rx="22.5vw"
            stroke="#D3D3D3"
          />
          <path
            fill-rule="evenodd"
            clip-rule="evenodd"
            d="M15.3938 15.3938C14.8687 15.9189 14.8687 16.7703 15.3938 17.2954L21.0985 23L15.3938 28.7046C14.8687 29.2297 14.8687 30.0811 15.3938 30.6062C15.9189 31.1313 16.7703 31.1313 17.2954 30.6062L23 24.9015L28.7046 30.6061C29.2297 31.1312 30.0811 31.1312 30.6062 30.6061C31.1313 30.081 31.1313 29.2297 30.6062 28.7046L24.9016 23L30.6062 17.2954C31.1313 16.7703 31.1313 15.919 30.6062 15.3939C30.0811 14.8688 29.2297 14.8688 28.7046 15.3939L23 21.0985L17.2954 15.3938C16.7703 14.8687 15.9189 14.8687 15.3938 15.3938Z"
            fill="#151515"
          />
        </svg>
      </div>
      <div className="error-body">
        <div className="error-body-text">
          <span className="error-body-text-h1">404</span>
          <span className="error-body-text-h2">Page not found!</span>
          <span className="error-body-text-p">
            We’re sorry, the page you requested could not be found. Please go
            back to the homepage!
          </span>
        </div>
        <button onClick={() => handlePage("home")} className="btn">
          GO HOME
        </button>
      </div>
    </div>
  );
}
