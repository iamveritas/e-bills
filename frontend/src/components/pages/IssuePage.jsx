import React from "react";
import IssueForm from "../sections/IssueForm";
export default function IssuePage({ changeHandle, identity, data, handlePage }) {
    return (
        <div className="issue">
            <IssueForm
                handlePage={handlePage}
                changeHandle={changeHandle}
                data={data}
                identity={identity}
            />
        </div>
    );
}
