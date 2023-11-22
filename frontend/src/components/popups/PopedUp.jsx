import React from "react";
import CheckBox from "../elements/CheckBox";

export default function PopedUp({filter, changleHandler}) {
    return (
        <div className="poped-up">
            <div className="filter-box">
                <CheckBox
                    name="imPayee"
                    text="If i am Payee"
                    checkCheck={filter?.imPayee}
                    changeListener={changleHandler}
                />
            </div>
            <hr/>
            <div className="filter-box">
                <CheckBox
                    name="imDrawer"
                    text="If i am Drawer"
                    checkCheck={filter?.imDrawer}
                    changeListener={changleHandler}
                />
            </div>
            <hr/>
            <div className="filter-box">
                <CheckBox
                    name="imDrawee"
                    text="If i am Drawee"
                    checkCheck={filter?.imDrawee}
                    changeListener={changleHandler}
                />
            </div>
        </div>
    );
}
