use std::path::Path;

use chrono::{Days, Utc};
use rocket::form::Form;
use rocket::{Request, State};
use rocket_dyn_templates::{context, handlebars, Template};

use crate::blockchain::{Chain, GossipsubEvent, GossipsubEventId, OperationCode};
use crate::constants::{BILLS_FOLDER_PATH, BILL_VALIDITY_PERIOD, IDENTITY_FILE_PATH};
use crate::dht::network::Client;
use crate::{
    accept_bill, add_in_contacts_map, blockchain, create_whole_identity,
    endorse_bill_and_return_new_holder_id, get_bills, get_whole_identity, issue_new_bill,
    read_bill_from_file, read_contacts_map, read_identity_from_file, read_peer_id_from_file,
    request_acceptance, AcceptBitcreditBillForm, BitcreditBill, BitcreditBillForm,
    EndorseBitcreditBillForm, Identity, IdentityForm, IdentityWithAll, NewContactForm,
    RequestToAcceptBitcreditBillForm,
};

use self::handlebars::{Handlebars, JsonRender};

#[get("/")]
pub async fn start() -> Template {
    if !Path::new(IDENTITY_FILE_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else {
        let bills = get_bills();
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
    let bills = get_bills();

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
        let bills = get_bills();

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
    } else if Path::new((BILLS_FOLDER_PATH.to_string() + "/" + &id + ".json").as_str()).exists() {
        let bill: BitcreditBill = read_bill_from_file(&id);

        let peer_id = read_peer_id_from_file();
        let str_peer_id = peer_id.to_string();

        let last_block = Chain::read_chain_from_file(&bill.name)
            .get_latest_block()
            .clone();
        let operation_code = last_block.operation_code;

        let identity: IdentityWithAll = get_whole_identity();

        Template::render(
            "hbs/bill",
            context! {
                codes: blockchain::OperationCode::get_all_operation_codes(),
                operation_code: operation_code,
                peer_id: str_peer_id,
                bill: Some(bill),
                identity: Some(identity.identity),
            },
        )
    } else {
        let bills = get_bills();
        let identity: IdentityWithAll = get_whole_identity();
        let peer_id = read_peer_id_from_file().to_string();

        Template::render(
            "hbs/home",
            context! {
                peer_id: Some(peer_id),
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

        let bills = get_bills();
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
pub async fn issue_bill(state: &State<Client>, bill_form: Form<BitcreditBillForm>) -> Template {
    if !Path::new(IDENTITY_FILE_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else {
        let bill = bill_form.into_inner();
        let drawer = read_identity_from_file();
        let bill = issue_new_bill(
            bill.bill_jurisdiction,
            bill.place_of_drawing,
            bill.amount_numbers,
            drawer.clone(),
            bill.language,
            bill.drawee_name,
        );

        let mut client = state.inner().clone();

        let mut nodes: Vec<String> = Vec::new();

        let my_peer_id = read_peer_id_from_file();
        nodes.push(my_peer_id.to_string());

        nodes.push(bill.drawee_name.clone());

        for node in nodes {
            if !node.is_empty() {
                println!("Add {} for node {}", &bill.name, &node);
                client.add_bill_to_dht_for_node(&bill.name, &node).await;
            }
        }

        client.subscribe_to_topic(bill.name.clone()).await;

        client.put(&bill.name).await;

        let bills = get_bills();
        let identity = read_identity_from_file();

        Template::render(
            "hbs/home",
            context! {
                identity: Some(identity),
                bills: bills,
            },
        )
    }
}

//TODO: change
#[post("/endorse", data = "<endorse_bill_form>")]
pub async fn endorse_bill(
    state: &State<Client>,
    endorse_bill_form: Form<EndorseBitcreditBillForm>,
) -> Template {
    if !Path::new(IDENTITY_FILE_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else {
        let mut client = state.inner().clone();

        let new_holder_id = endorse_bill_and_return_new_holder_id(
            &endorse_bill_form.bill_name,
            endorse_bill_form.new_holder.clone(),
        );
        if !new_holder_id.is_empty() {
            let chain: Chain = Chain::read_chain_from_file(&endorse_bill_form.bill_name);
            let block = chain.get_latest_block();

            let block_bytes = serde_json::to_vec(block).expect("Error serializing block");
            let event = GossipsubEvent::new(GossipsubEventId::Block, block_bytes);
            let message = event.to_byte_array();

            client
                .add_message_to_topic(message, endorse_bill_form.bill_name.clone())
                .await;

            client
                .add_bill_to_dht_for_node(&endorse_bill_form.bill_name, &new_holder_id)
                .await;
        }

        let bills = get_bills();
        let identity: Identity = read_identity_from_file();

        Template::render(
            "hbs/home",
            context! {
                identity: Some(identity),
                bills: bills,
            },
        )
    }
}

#[post("/request_to_accept", data = "<request_to_accept_bill_form>")]
pub async fn request_to_accept_bill(
    state: &State<Client>,
    request_to_accept_bill_form: Form<RequestToAcceptBitcreditBillForm>,
) -> Template {
    if !Path::new(IDENTITY_FILE_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else {
        let mut client = state.inner().clone();

        let correct = request_acceptance(&request_to_accept_bill_form.bill_name);

        if correct {
            let chain: Chain = Chain::read_chain_from_file(&request_to_accept_bill_form.bill_name);
            let block = chain.get_latest_block();

            let block_bytes = serde_json::to_vec(block).expect("Error serializing block");
            let event = GossipsubEvent::new(GossipsubEventId::Block, block_bytes);
            let message = event.to_byte_array();

            client
                .add_message_to_topic(message, request_to_accept_bill_form.bill_name.clone())
                .await;
        }

        let bills = get_bills();
        let identity: Identity = read_identity_from_file();

        Template::render(
            "hbs/home",
            context! {
                identity: Some(identity),
                bills: bills,
            },
        )
    }
}

#[post("/accept", data = "<accept_bill_form>")]
pub async fn accept_bill_form(
    state: &State<Client>,
    accept_bill_form: Form<AcceptBitcreditBillForm>,
) -> Template {
    if !Path::new(IDENTITY_FILE_PATH).exists() {
        Template::render("hbs/create_identity", context! {})
    } else {
        let mut client = state.inner().clone();

        let mut correct = false;

        if accept_bill_form
            .operation_code
            .eq(&OperationCode::Accept.get_string_from_operation_code())
        {
            correct = accept_bill(&accept_bill_form.bill_name);
        }

        if correct {
            let chain: Chain = Chain::read_chain_from_file(&accept_bill_form.bill_name);
            let block = chain.get_latest_block();

            let block_bytes = serde_json::to_vec(block).expect("Error serializing block");
            let event = GossipsubEvent::new(GossipsubEventId::Block, block_bytes);
            let message = event.to_byte_array();

            client
                .add_message_to_topic(message, accept_bill_form.bill_name.clone())
                .await;
        }

        let bills = get_bills();
        let identity: Identity = read_identity_from_file();

        Template::render(
            "hbs/home",
            context! {
                identity: Some(identity),
                bills: bills,
            },
        )
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
