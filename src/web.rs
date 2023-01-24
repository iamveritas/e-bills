use self::handlebars::{Handlebars, JsonRender};
use crate::constants::{BILLS_FOLDER_PATH, BILL_VALIDITY_PERIOD, IDENTITY_FOLDER_PATH};
use crate::{
    create_whole_identity, get_whole_identity, issue_new_bill, read_bill_from_file,
    read_identity_from_file, BitcreditBill, BitcreditBillForm, Identity, IdentityForm,
    IdentityWithAll,
};
use chrono::{Days, Utc};
use rocket::form::Form;
use rocket::Request;
use rocket_dyn_templates::{context, handlebars, Template};
use std::fs;
use std::path::Path;

#[get("/")]
pub async fn start() -> Template {
    if !Path::new(IDENTITY_FOLDER_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else {
        let bills = bills();
        let identity: IdentityWithAll = get_whole_identity();

        Template::render(
            "hbs/home",
            context! {
                identity: Some(identity.identity),
                bills: bills,
            },
        )
    }
}

#[get("/")]
pub async fn info() -> Template {
    Template::render("hbs/info", context! {})
}

#[get("/")]
pub async fn get_identity() -> Template {
    if !Path::new(IDENTITY_FOLDER_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else {
        let identity: IdentityWithAll = get_whole_identity();
        let peer_id = identity.peer_id.to_string();
        Template::render(
            "hbs/identity",
            context! {
                peer_id: peer_id,
                identity: Some(identity.identity),
            },
        )
    }
}

#[post("/create", data = "<identity_form>")]
pub async fn create_identity(identity_form: Form<IdentityForm>) -> Template {
    let identity: IdentityForm = identity_form.into_inner();
    create_whole_identity(
        identity.name,
        identity.date_of_birth,
        identity.city_of_birth,
        identity.country_of_birth,
        identity.email,
        identity.postal_address,
    );
    let identity: IdentityWithAll = get_whole_identity();
    let bills = bills();

    Template::render(
        "hbs/home",
        context! {
            identity: Some(identity.identity),
            bills: bills,
        },
    )
}

#[get("/")]
pub async fn bills_list() -> Template {
    if !Path::new(IDENTITY_FOLDER_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else {
        let bills = bills();

        Template::render(
            "hbs/bills_list",
            context! {
                bills: bills,
            },
        )
    }
}

#[get("/<id>")]
pub async fn get_bill(id: String) -> Template {
    if !Path::new(IDENTITY_FOLDER_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else if Path::new((BILLS_FOLDER_PATH.to_string() + &"/".to_string() + &id).as_str()).exists()
    {
        let bill: BitcreditBill = read_bill_from_file(&id);

        Template::render(
            "hbs/bill",
            context! {
                bill: Some(bill),
            },
        )
    } else {
        let bills = bills();
        let identity: IdentityWithAll = get_whole_identity();

        Template::render(
            "hbs/home",
            context! {
                identity: Some(identity.identity),
                bills: bills,
            },
        )
    }
}

#[get("/")]
pub async fn new_bill() -> Template {
    if !Path::new(IDENTITY_FOLDER_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else {
        let identity: IdentityWithAll = get_whole_identity();
        let utc = Utc::now();
        let date_of_issue = utc.naive_local().date().to_string();
        let maturity_date = utc
            .checked_add_days(Days::new(BILL_VALIDITY_PERIOD))
            .unwrap()
            .naive_local()
            .date()
            .to_string();
        Template::render(
            "hbs/new_bill",
            context! {
                identity: Some(identity.identity),
                date_of_issue: date_of_issue,
                maturity_date: maturity_date,
            },
        )
    }
}

#[post("/issue", data = "<bill_form>")]
pub async fn issue_bill(bill_form: Form<BitcreditBillForm>) -> Template {
    if !Path::new(IDENTITY_FOLDER_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else {
        let bill: BitcreditBillForm = bill_form.into_inner();
        let drawer: Identity = read_identity_from_file();
        let bill = issue_new_bill(
            bill.bill_jurisdiction,
            bill.place_of_drawing,
            bill.amount_numbers,
            drawer,
            bill.language,
            bill.drawee_name,
        );

        let bill: BitcreditBill = read_bill_from_file(&bill.name);

        Template::render(
            "hbs/bill",
            context! {
                bill: Some(bill),
            },
        )
    }
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("We couldn't find the requested path '{}'", req.uri())
}

fn bills() -> Vec<BitcreditBill> {
    let mut bills = Vec::new();
    let paths = fs::read_dir(BILLS_FOLDER_PATH).unwrap();
    for _path in paths {
        let mut path = _path.unwrap().path().display().to_string();
        let path_vec = path.split("/").collect::<Vec<&str>>();
        path = path_vec[1].to_string();
        let bill = read_bill_from_file(&path);
        bills.push(bill);
    }
    bills
}

pub fn customize(hbs: &mut Handlebars) {
    hbs.register_helper("wow", Box::new(wow_helper));
    hbs.register_template_string(
        "hbs/about.html",
        r#"
        {{#*inline "page"}}
        <section id="about">
        <h1>About - Here's another page!</h1>
        </section>
        {{/inline}}
        {{> hbs/layout}}
    "#,
    )
    .expect("valid HBS template");
}

fn wow_helper(
    h: &handlebars::Helper<'_, '_>,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext<'_, '_>,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
    }

    Ok(())
}
