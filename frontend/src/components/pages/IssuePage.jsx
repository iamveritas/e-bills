import React from "react";
import IssueForm from "../sections/IssueForm";
export default function IssuePage({ contacts, changeHandle, identity, data, handlePage }) {
    return (
        <div className="issue">
            <IssueForm
                contacts={contacts}
                handlePage={handlePage}
                changeHandle={changeHandle}
                data={data}
                identity={identity}
            />
        </div>
    );
}
