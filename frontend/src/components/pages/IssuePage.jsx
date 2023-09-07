import React from "react";
import IssueForm from "../sections/IssueForm";
export default function IssuePage({ contacts, handlePage, changeHandle, handleChangeDrawerIsDrawee, handleChangeDrawerIsPayee, data, identity }) {
    return (
        <div className="issue">
            <IssueForm
                contacts={contacts}
                handlePage={handlePage}
                changeHandle={changeHandle}
                handleChangeDrawerIsDrawee={handleChangeDrawerIsDrawee}
                handleChangeDrawerIsPayee={handleChangeDrawerIsPayee}
                data={data}
                identity={identity}
            />
        </div>
    );
}
