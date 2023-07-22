import React from "react";

export default function UniqueNumber({ UID, date }) {
    return (
        <div className="issue-number">
            <span className="issue-number-holder">
                <span>Nr:</span>
                <span className="colored">{UID}</span>
            </span>
            <span className="issue-number-date">{date}</span>
        </div>
    );
}
