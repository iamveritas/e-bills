use std::collections::HashMap;
use std::fs;
use std::path::Path;

use chrono::{Days, Utc};
use rocket::form::Form;
use rocket::{Request, State};
use rocket_dyn_templates::{context, handlebars, Template};

use crate::constants::{BILLS_FOLDER_PATH, BILL_VALIDITY_PERIOD, IDENTITY_FILE_PATH};
use crate::dht::network::Client;
use crate::{
    add_in_contacts_map, create_whole_identity, endorse_bill_to_new_holder_and_return_his_node_id,
    get_all_nodes_from_bill, get_whole_identity, hash_bill, issue_new_bill, read_bill_from_file,
    read_contacts_map, read_identity_from_file, read_peer_id_from_file, BitcreditBill,
    BitcreditBillForm, EndorseBitcreditBillForm, IdentityForm, IdentityWithAll, NewContactForm,
};

use self::handlebars::{Handlebars, JsonRender};

#[get("/")]
pub async fn start() -> Template {
    if !Path::new(IDENTITY_FILE_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else {
        let bills = bills();
        let identity: IdentityWithAll = get_whole_identity();

        Template::render(
            "hbs/home",
            context! {
                identity: Some(identity.identity),
                //TODO: change
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
    if !Path::new(IDENTITY_FILE_PATH).exists() {
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
    if !Path::new(IDENTITY_FILE_PATH).exists() {
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
    if !Path::new(IDENTITY_FILE_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else if Path::new((BILLS_FOLDER_PATH.to_string() + "/" + &id).as_str()).exists() {
        let bill: BitcreditBill = read_bill_from_file(&id, &"delete".to_string());

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

#[get("/dht")]
pub async fn search_bill(state: &State<Client>) -> Template {
    if !Path::new(IDENTITY_FILE_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else {
        let mut client = state.inner().clone();
        let local_peer_id = read_peer_id_from_file();
        client.check_new_bills(local_peer_id.to_string()).await;

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
    if !Path::new(IDENTITY_FILE_PATH).exists() {
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
pub async fn issue_bill(state: &State<Client>, bill_form: Form<BitcreditBillForm>) {
    // if !Path::new(IDENTITY_FILE_PATH).exists() {
    //     Template::render("hbs/create_identity", context! {})
    // } else {
    let bill = bill_form.into_inner();
    let drawer = read_identity_from_file();
    let (name_bill, bill) = issue_new_bill(
        bill.bill_jurisdiction,
        bill.place_of_drawing,
        bill.amount_numbers,
        drawer,
        bill.language,
        bill.drawee_name,
    );

    let mut client = state.inner().clone();

    let readeble_name = hash_bill(&bill);
    let mut nodes = get_all_nodes_from_bill(&name_bill, &readeble_name);

    let my_peer_id = read_peer_id_from_file();
    nodes.push(my_peer_id.to_string());

    for node in nodes {
        println!("Add {} for node {}", name_bill, node);
        client.add_bill_to_dht_for_node(&name_bill, node).await;
    }

    client.subscribe_to_topic(name_bill.clone()).await;

    client.put(&name_bill).await;

    // Template::render(
    //     "hbs/bill",
    //     context! {
    //         bill: Some(bill),
    //     },
    // )
    // }
}

#[post("/endorse", data = "<endorse_bill_form>")]
pub async fn endorse_bill(
    state: &State<Client>,
    endorse_bill_form: Form<EndorseBitcreditBillForm>,
) {
    let mut client = state.inner().clone();

    let node_id = endorse_bill_to_new_holder_and_return_his_node_id(
        &endorse_bill_form.bill_name,
        &endorse_bill_form.readable_hash_name,
        endorse_bill_form.new_holder.clone(),
    );

    if !node_id.is_empty() {
        client
            .add_message_to_topic(
                node_id.as_bytes().to_vec(),
                endorse_bill_form.bill_name.clone(),
            )
            .await;

        client
            .add_bill_to_dht_for_node(&endorse_bill_form.bill_name, node_id)
            .await;
    }
}

pub fn add_to_nodes(map: &HashMap<String, String>, node: &String, nodes: &mut Vec<String>) {
    let mut node_id = "";
    if map.contains_key(node) {
        node_id = map.get(node).expect("Contact not found");
    }
    if !node_id.is_empty() {
        nodes.push(node_id.to_string());
    }
}

#[get("/add")]
pub async fn add_contact() -> Template {
    if !Path::new(IDENTITY_FILE_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else {
        Template::render("hbs/new_contact", context! {})
    }
}

#[post("/new", data = "<new_contact_form>")]
pub async fn new_contact(new_contact_form: Form<NewContactForm>) -> Template {
    if !Path::new(IDENTITY_FILE_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else {
        add_in_contacts_map(
            new_contact_form.name.clone(),
            new_contact_form.node_id.clone(),
        );

        let map = read_contacts_map();

        Template::render(
            "hbs/contacts",
            context! {
                contacts: map,
            },
        )
    }
}

#[get("/")]
pub async fn contacts() -> Template {
    if !Path::new(IDENTITY_FILE_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else {
        let contacts = read_contacts_map();
        Template::render(
            "hbs/contacts",
            context! {
                contacts: contacts,
            },
        )
    }
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("We couldn't find the requested path '{}'", req.uri())
}

//TODO: change
fn bills() -> Vec<BitcreditBill> {
    let mut bills = Vec::new();
    let paths = fs::read_dir(BILLS_FOLDER_PATH).unwrap();
    for _path in paths {
        let mut path = _path.unwrap().path().display().to_string();
        let path_vec = path.split('/').collect::<Vec<&str>>();
        path = path_vec[1].to_string();
        let bill = read_bill_from_file(&path, &"delete".to_string());
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
