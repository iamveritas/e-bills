import React, {useContext} from "react";
import IssueForm from "../sections/IssueForm";
import Header from "../sections/Header";
import TopDownHeading from "../elements/TopDownHeading";
import IconHolder from "../elements/IconHolder";
import attachment from "../../assests/attachment.svg";
import {MainContext} from "../../context/MainContext";

export default function IssuePage({
                                      changeHandle,
                                      data,
                                      handleChangeDrawerIsDrawee,
                                      handleChangeDrawerIsPayee,
                                  }) {
    const {identity, contacts, handlePage} = useContext(MainContext);
    return (
        <div className="issue">
            <Header title="Issue"/>
            {/*<UniqueNumber UID="001" date="16-Feb-2023" />*/}
            <div className="head">
                <TopDownHeading upper="Against this" lower="Bill Of Exchange"/>
                <IconHolder icon={attachment}/>
            </div>
            <IssueForm
                contacts={contacts}
                handlePage={handlePage}
                changeHandle={changeHandle}
                data={data}
                identity={identity}
                handleChangeDrawerIsDrawee={handleChangeDrawerIsDrawee}
                handleChangeDrawerIsPayee={handleChangeDrawerIsPayee}
            />
        </div>
    );
}
