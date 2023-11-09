import React from "react";

export default function PopedUp({ filter, changleHandler }) {
  return (
    <div className="poped-up">
      <div className="filter-box">
        <span className="filter-box-input">
          <input
            id="imPayee"
            name="imPayee"
            type="checkbox"
            onChange={changleHandler}
          />
          <label
            htmlFor="imPayee"
            className="check-boxes"
            style={{
              borderColor: `#${filter?.imPayee ? "F7931A" : "545454"}`,
            }}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="15"
              height="12"
              viewBox="0 0 15 12"
              fill="none"
            >
              <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M14.1757 0.762852C14.5828 1.13604 14.6104 1.76861 14.2372 2.17573L5.98716 11.1757C5.79775 11.3824 5.53031 11.5 5.25001 11.5C4.9697 11.5 4.70226 11.3824 4.51285 11.1757L0.762852 7.08482C0.389659 6.6777 0.417162 6.04514 0.824281 5.67194C1.2314 5.29875 1.86397 5.32625 2.23716 5.73337L5.25001 9.02011L12.7629 0.824281C13.136 0.417162 13.7686 0.389659 14.1757 0.762852Z"
                fill={`#${filter?.imPayee ? "F7931A" : "545454"}`}
              />
            </svg>
          </label>
          <span className="filter-box-input-text">If i am Payee</span>
        </span>
      </div>
      <hr />
      <div className="filter-box">
        <span className="filter-box-input">
          <input
            id="imDrawer"
            name="imDrawer"
            type="checkbox"
            onChange={changleHandler}
          />
          <label
            htmlFor="imDrawer"
            className="check-boxes"
            style={{
              borderColor: `#${filter?.imDrawer ? "F7931A" : "545454"}`,
            }}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="15"
              height="12"
              viewBox="0 0 15 12"
              fill="none"
            >
              <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M14.1757 0.762852C14.5828 1.13604 14.6104 1.76861 14.2372 2.17573L5.98716 11.1757C5.79775 11.3824 5.53031 11.5 5.25001 11.5C4.9697 11.5 4.70226 11.3824 4.51285 11.1757L0.762852 7.08482C0.389659 6.6777 0.417162 6.04514 0.824281 5.67194C1.2314 5.29875 1.86397 5.32625 2.23716 5.73337L5.25001 9.02011L12.7629 0.824281C13.136 0.417162 13.7686 0.389659 14.1757 0.762852Z"
                fill={`#${filter?.imDrawer ? "F7931A" : "545454"}`}
              />
            </svg>
          </label>
          <span className="filter-box-input-text">If i am Drawee</span>
        </span>
      </div>
      <hr />
      <div className="filter-box">
        <span className="filter-box-input">
          <input
            id="imDrawee"
            name="imDrawee"
            type="checkbox"
            onChange={changleHandler}
          />
          <label
            htmlFor="imDrawee"
            className="check-boxes"
            style={{
              borderColor: `#${filter?.imDrawee ? "F7931A" : "545454"}`,
            }}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="15"
              height="12"
              viewBox="0 0 15 12"
              fill="none"
            >
              <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M14.1757 0.762852C14.5828 1.13604 14.6104 1.76861 14.2372 2.17573L5.98716 11.1757C5.79775 11.3824 5.53031 11.5 5.25001 11.5C4.9697 11.5 4.70226 11.3824 4.51285 11.1757L0.762852 7.08482C0.389659 6.6777 0.417162 6.04514 0.824281 5.67194C1.2314 5.29875 1.86397 5.32625 2.23716 5.73337L5.25001 9.02011L12.7629 0.824281C13.136 0.417162 13.7686 0.389659 14.1757 0.762852Z"
                fill={`#${filter?.imDrawee ? "F7931A" : "545454"}`}
              />
            </svg>
          </label>
          <span className="filter-box-input-text">If i am Drawer</span>
        </span>
      </div>
    </div>
  );
}
