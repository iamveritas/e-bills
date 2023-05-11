extern crate core;
#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::path::Path;
use std::{env, fs, mem, path};

use borsh::{self, BorshDeserialize, BorshSerialize};
use chrono::{Days, Utc};
use libp2p::identity::Keypair;
use libp2p::PeerId;
use openssl::pkey::{Private, Public};
use openssl::rsa;
use openssl::rsa::{Padding, Rsa};
use openssl::sha::sha256;
use rocket::fs::FileServer;
use rocket::serde::{Deserialize, Serialize};
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

use crate::blockchain::{start_blockchain_for_new_bill, Block, Chain, OperationCode};
use crate::constants::{
    BILLS_FOLDER_PATH, BILL_VALIDITY_PERIOD, BOOTSTRAP_FOLDER_PATH, BTC,
    COMPOUNDING_INTEREST_RATE_ZERO, CONTACT_MAP_FILE_PATH, CONTACT_MAP_FOLDER_PATH,
    CSS_FOLDER_PATH, IDENTITY_ED_25529_KEYS_FILE_PATH, IDENTITY_FILE_PATH, IDENTITY_FOLDER_PATH,
    IDENTITY_PEER_ID_FILE_PATH, IMAGE_FOLDER_PATH, TEMPLATES_FOLDER_PATH,
};
use crate::numbers_to_words::encode;
use crate::payments::generate_address_to_pay;

mod blockchain;
mod constants;
mod dht;
mod numbers_to_words;
mod payments;
mod test;
mod web;

// MAIN
// #[rocket::main]
#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    env_logger::init();

    init_folders();

    let mut dht = dht::dht_main().await.expect("DHT failed to start");

    let local_peer_id = read_peer_id_from_file();
    dht.check_new_bills(local_peer_id.to_string().clone()).await;
    dht.upgrade_table(local_peer_id.to_string().clone()).await;
    dht.subscribe_to_all_bills_topics().await;
    dht.start_provide().await;
    dht.receive_updates_for_all_bills_topics().await;
    let _rocket = rocket_main(dht).launch().await.unwrap();
}

fn rocket_main(dht: dht::network::Client) -> Rocket<Build> {
    let rocket = rocket::build()
        .register("/", catchers![web::not_found])
        .manage(dht)
        .mount("/image", FileServer::from(IMAGE_FOLDER_PATH))
        .mount("/css", FileServer::from(CSS_FOLDER_PATH))
        .mount("/", routes![web::start])
        .mount(
            "/identity",
            routes![web::get_identity, web::create_identity,],
        )
        .mount("/bills", routes![web::bills_list])
        .mount("/info", routes![web::info])
        .mount(
            "/contacts",
            routes![web::add_contact, web::new_contact, web::contacts],
        )
        .mount(
            "/bill",
            routes![
                web::get_bill,
                web::issue_bill,
                web::endorse_bill,
                web::new_bill,
                web::search_bill,
                web::request_to_accept_bill,
                web::accept_bill_form,
                web::request_to_pay_bill,
            ],
        )
        .attach(Template::custom(|engines| {
            web::customize(&mut engines.handlebars);
        }));

    // open::that("http://127.0.0.1:8000").expect("Can't open browser.");

    rocket
}

fn init_folders() {
    if !Path::new(CONTACT_MAP_FOLDER_PATH).exists() {
        fs::create_dir(CONTACT_MAP_FOLDER_PATH).expect("Can't create folder contacts.");
    }
    if !Path::new(IDENTITY_FOLDER_PATH).exists() {
        fs::create_dir(IDENTITY_FOLDER_PATH).expect("Can't create folder identity.");
    }
    if !Path::new(BILLS_FOLDER_PATH).exists() {
        fs::create_dir(BILLS_FOLDER_PATH).expect("Can't create folder bills.");
    }
    if !Path::new(CSS_FOLDER_PATH).exists() {
        fs::create_dir(CSS_FOLDER_PATH).expect("Can't create folder css.");
    }
    if !Path::new(IMAGE_FOLDER_PATH).exists() {
        fs::create_dir(IMAGE_FOLDER_PATH).expect("Can't create folder image.");
    }
    if !Path::new(TEMPLATES_FOLDER_PATH).exists() {
        fs::create_dir(TEMPLATES_FOLDER_PATH).expect("Can't create folder templates.");
    }
    if !Path::new(BOOTSTRAP_FOLDER_PATH).exists() {
        fs::create_dir(BOOTSTRAP_FOLDER_PATH).expect("Can't create folder templates.");
    }
}

//-------------------------Contacts map-------------------------
fn read_contacts_map() -> HashMap<String, String> {
    if !Path::new(CONTACT_MAP_FILE_PATH).exists() {
        create_contacts_map();
    }
    let data: Vec<u8> = fs::read(CONTACT_MAP_FILE_PATH).expect("Unable to read contacts.");
    let contacts: HashMap<String, String> = HashMap::try_from_slice(&data).unwrap();
    contacts
}

fn add_in_contacts_map(name: String, peer_id: String) {
    if !Path::new(CONTACT_MAP_FILE_PATH).exists() {
        create_contacts_map();
    }
    let mut contacts: HashMap<String, String> = read_contacts_map();
    contacts.insert(name, peer_id);
    write_contacts_map(contacts);
}

fn create_contacts_map() {
    let contacts: HashMap<String, String> = HashMap::new();
    write_contacts_map(contacts);
}

fn write_contacts_map(map: HashMap<String, String>) {
    let contacts_byte = map.try_to_vec().unwrap();
    fs::write(CONTACT_MAP_FILE_PATH, contacts_byte).expect("Unable to write peer id in file.");
}

fn get_contact_from_map(name: &String) -> String {
    let contacts = read_contacts_map();
    if contacts.contains_key(name) {
        contacts.get(name).unwrap().to_string()
    } else {
        String::new()
    }
}
//--------------------------------------------------------------

//-------------------------RSA----------------------------------
fn generation_rsa_key() -> Rsa<Private> {
    Rsa::generate(2048).unwrap()
}

fn pem_private_key_from_rsa(rsa: &Rsa<Private>) -> String {
    let private_key: Vec<u8> = rsa.private_key_to_pem().unwrap();
    String::from_utf8(private_key).unwrap()
}

fn pem_public_key_from_rsa(rsa: &Rsa<Private>) -> String {
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();
    String::from_utf8(public_key).unwrap()
}

fn private_key_from_pem_u8(private_key_u8: &Vec<u8>) -> Rsa<Private> {
    rsa::Rsa::private_key_from_pem(private_key_u8).unwrap()
}

fn public_key_from_pem_u8(public_key_u8: &Vec<u8>) -> Rsa<Public> {
    rsa::Rsa::public_key_from_pem(public_key_u8).unwrap()
}
//--------------------------------------------------------------

//-------------------------Bytes common-------------------------
fn encrypt_bytes(bytes: &Vec<u8>, rsa_key: &Rsa<Private>) -> Vec<u8> {
    let key_size: usize = (rsa_key.size() / 2) as usize; //128

    let mut whole_encrypted_buff: Vec<u8> = Vec::new();
    let mut temp_buff: Vec<u8> = vec![0; key_size];
    let mut temp_buff_encrypted: Vec<u8> = vec![0; rsa_key.size() as usize];

    let number_of_key_size_in_whole_bill: usize = bytes.len() / key_size;
    let remainder: usize = bytes.len() - key_size * number_of_key_size_in_whole_bill;

    for i in 0..number_of_key_size_in_whole_bill {
        for j in 0..key_size {
            let byte_number: usize = key_size * i + j;
            temp_buff[j] = bytes[byte_number];
        }

        let _encrypted_len: usize = rsa_key
            .public_encrypt(&temp_buff, &mut temp_buff_encrypted, Padding::PKCS1)
            .unwrap();

        whole_encrypted_buff.append(&mut temp_buff_encrypted);
        temp_buff = vec![0; key_size];
        temp_buff_encrypted = vec![0; rsa_key.size() as usize];
    }

    if remainder != 0 {
        temp_buff = vec![0; remainder];

        let position: usize = key_size * number_of_key_size_in_whole_bill;
        let mut index_in_temp_buff: usize = 0;

        for i in position..bytes.len() {
            temp_buff[index_in_temp_buff] = bytes[i];
            index_in_temp_buff += 1;
        }

        index_in_temp_buff = 0;

        let _encrypted_len: usize = rsa_key
            .public_encrypt(&temp_buff, &mut temp_buff_encrypted, Padding::PKCS1)
            .unwrap();

        whole_encrypted_buff.append(&mut temp_buff_encrypted);
        temp_buff.clear();
        temp_buff_encrypted.clear();
    }

    whole_encrypted_buff
}

fn decrypt_bytes(bytes: &Vec<u8>, rsa_key: &Rsa<Private>) -> Vec<u8> {
    let key_size: usize = rsa_key.size() as usize; //256

    let mut whole_decrypted_buff: Vec<u8> = Vec::new();
    let mut temp_buff: Vec<u8> = vec![0; rsa_key.size() as usize];
    let mut temp_buff_decrypted: Vec<u8> = vec![0; rsa_key.size() as usize];

    let number_of_key_size_in_whole_bill: usize = bytes.len() / key_size;
    // let remainder = bill_bytes.len() - key_size * number_of_key_size_in_whole_bill;

    for i in 0..number_of_key_size_in_whole_bill {
        for j in 0..key_size {
            let byte_number = key_size * i + j;
            temp_buff[j] = bytes[byte_number];
        }

        let decrypted_len: usize = rsa_key
            .private_decrypt(&temp_buff, &mut temp_buff_decrypted, Padding::PKCS1)
            .unwrap();

        whole_decrypted_buff.append(&mut temp_buff_decrypted[0..decrypted_len].to_vec());
        temp_buff = vec![0; rsa_key.size() as usize];
        temp_buff_decrypted = vec![0; rsa_key.size() as usize];
    }

    // if remainder != 0 {
    //     let position = key_size * number_of_key_size_in_whole_bill;
    //     let mut index_in_temp_buff = 0;
    //
    //     for i in position..bill_bytes.len() {
    //         temp_buff[index_in_temp_buff] = bill_bytes[i];
    //         index_in_temp_buff = index_in_temp_buff + 1;
    //     }
    //
    //     index_in_temp_buff = 0;
    //
    //     let decrypted_len = rsa_key
    //         .public_decrypt(&*temp_buff, &mut temp_buff_decrypted, Padding::PKCS1)
    //         .unwrap();
    //
    //     whole_decrypted_buff.append(&mut temp_buff_decrypted);
    //     temp_buff.clear();
    //     temp_buff_decrypted.clear();
    // }

    whole_decrypted_buff
}

unsafe fn structure_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}
//--------------------------------------------------------------

//-------------------------Identity-----------------------------
pub struct IdentityWithAll {
    identity: Identity,
    peer_id: PeerId,
    key_pair: Keypair,
}

#[derive(BorshSerialize, BorshDeserialize, FromForm, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Identity {
    name: String,
    date_of_birth: String,
    city_of_birth: String,
    country_of_birth: String,
    email: String,
    postal_address: String,
    public_key_pem: String,
    private_key_pem: String,
}

pub fn get_whole_identity() -> IdentityWithAll {
    let identity: Identity = read_identity_from_file();
    let ed25519_keys: Keypair = read_ed25519_keypair_from_file();
    let peer_id: PeerId = read_peer_id_from_file();

    IdentityWithAll {
        identity,
        peer_id,
        key_pair: ed25519_keys,
    }
}

pub fn create_whole_identity(
    name: String,
    date_of_birth: String,
    city_of_birth: String,
    country_of_birth: String,
    email: String,
    postal_address: String,
) -> IdentityWithAll {
    let identity = create_new_identity(
        name,
        date_of_birth,
        city_of_birth,
        country_of_birth,
        email,
        postal_address,
    );

    let ed25519_keys = read_ed25519_keypair_from_file();
    let peer_id = read_peer_id_from_file();

    write_identity_to_file(&identity);

    IdentityWithAll {
        identity,
        peer_id,
        key_pair: ed25519_keys,
    }
}

pub fn generate_dht_logic() {
    let ed25519_keys = Keypair::generate_ed25519();
    let peer_id = ed25519_keys.public().to_peer_id();
    write_dht_logic(&peer_id, &ed25519_keys);
}

fn write_dht_logic(peer_id: &PeerId, ed25519_keys: &Keypair) {
    write_peer_id_to_file(peer_id);
    write_ed25519_keypair_to_file(ed25519_keys);
}

fn create_new_identity(
    name: String,
    date_of_birth: String,
    city_of_birth: String,
    country_of_birth: String,
    email: String,
    postal_address: String,
) -> Identity {
    let rsa: Rsa<Private> = generation_rsa_key();
    let private_key: String = pem_private_key_from_rsa(&rsa);
    let public_key: String = pem_public_key_from_rsa(&rsa);

    Identity {
        name,
        date_of_birth,
        city_of_birth,
        country_of_birth,
        email,
        postal_address,
        public_key_pem: public_key,
        private_key_pem: private_key,
    }
}

fn write_identity_to_file(identity: &Identity) {
    let data: Vec<u8> = identity_to_byte_array(identity);
    fs::write(IDENTITY_FILE_PATH, data).expect("Unable to write file identity");
}

fn write_ed25519_keypair_to_file(ed25519_keys: &Keypair) {
    let data: &[u8] = unsafe { structure_as_u8_slice(ed25519_keys) };
    let data_sized = byte_array_to_size_array_keypair(data);
    fs::write(IDENTITY_ED_25529_KEYS_FILE_PATH, *data_sized)
        .expect("Unable to write keypair ed25519 in file");
}

fn write_peer_id_to_file(peer_id: &PeerId) {
    let data: &[u8] = unsafe { structure_as_u8_slice(peer_id) };
    let data_sized = byte_array_to_size_array_peer_id(data);
    fs::write(IDENTITY_PEER_ID_FILE_PATH, *data_sized).expect("Unable to write peer id in file");
}

fn read_identity_from_file() -> Identity {
    let data: Vec<u8> = fs::read(IDENTITY_FILE_PATH).expect("Unable to read file identity");
    identity_from_byte_array(&data)
}

fn read_ed25519_keypair_from_file() -> Keypair {
    let data: Vec<u8> =
        fs::read(IDENTITY_ED_25529_KEYS_FILE_PATH).expect("Unable to read file keypair");
    let key_pair_bytes_sized = byte_array_to_size_array_keypair(data.as_slice());
    let key_pair: Keypair = unsafe { mem::transmute_copy(key_pair_bytes_sized) };
    key_pair
}

fn read_peer_id_from_file() -> PeerId {
    let data: Vec<u8> =
        fs::read(IDENTITY_PEER_ID_FILE_PATH).expect("Unable to read file with peer id");
    let peer_id_bytes_sized = byte_array_to_size_array_peer_id(data.as_slice());
    let peer_id: PeerId = unsafe { mem::transmute_copy(peer_id_bytes_sized) };
    peer_id
}

fn identity_to_byte_array(identity: &Identity) -> Vec<u8> {
    identity.try_to_vec().unwrap()
}

fn identity_from_byte_array(identity: &Vec<u8>) -> Identity {
    Identity::try_from_slice(identity).unwrap()
}

fn byte_array_to_size_array_keypair(array: &[u8]) -> &[u8; ::std::mem::size_of::<Keypair>()] {
    array.try_into().expect("slice with incorrect length")
}

fn byte_array_to_size_array_peer_id(array: &[u8]) -> &[u8; ::std::mem::size_of::<PeerId>()] {
    array.try_into().expect("slice with incorrect length")
}
//--------------------------------------------------------------

//-------------------------Bill---------------------------------
#[derive(BorshSerialize, BorshDeserialize, FromForm, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct BitcreditBill {
    name: String,
    to_payee: bool,
    bill_jurisdiction: String,
    timestamp_at_drawing: i64,
    // The party obliged to pay a Bill
    drawee_name: String,
    // The party issuing a Bill
    drawer_name: String,
    // The person to whom the Payee or a prior holder endorses a bill
    holder_name: String,
    // Default - the drawer’s address.
    place_of_drawing: String,
    currency_code: String,
    //TODO: f64
    amount_numbers: u64,
    amounts_letters: String,
    maturity_date: String,
    date_of_issue: String,
    compounding_interest_rate: u64,
    type_of_interest_calculation: bool,
    // Defaulting to the drawee’s id/ address.
    place_of_payment: String,
    public_key_pem: String,
    private_key_pem: String,
    language: String,
}

//TODO change
impl BitcreditBill {
    pub fn new() -> Self {
        BitcreditBill {
            name: String::new(),
            to_payee: false,
            bill_jurisdiction: String::new(),
            timestamp_at_drawing: 0,
            drawee_name: String::new(),
            drawer_name: String::new(),
            holder_name: String::new(),
            place_of_drawing: String::new(),
            currency_code: String::new(),
            amount_numbers: 0,
            amounts_letters: String::new(),
            maturity_date: String::new(),
            date_of_issue: String::new(),
            compounding_interest_rate: 0,
            type_of_interest_calculation: false,
            place_of_payment: String::new(),
            public_key_pem: String::new(),
            private_key_pem: String::new(),
            language: String::new(),
        }
    }
}

pub fn issue_new_bill(
    bill_jurisdiction: String,
    place_of_drawing: String,
    amount_numbers: u64,
    drawer: Identity,
    language: String,
    drawee_name: String,
) -> BitcreditBill {
    let rsa: Rsa<Private> = generation_rsa_key();
    let bill_name: String = create_bill_name(&rsa);

    //This if need for no duplicate bill name.
    if Path::new((BILLS_FOLDER_PATH.to_string() + "/" + &bill_name).as_str()).exists() {
        issue_new_bill(
            bill_jurisdiction,
            place_of_drawing,
            amount_numbers,
            drawer,
            language,
            drawee_name,
        )
    } else {
        let private_key_pem: String = pem_private_key_from_rsa(&rsa);
        let public_key_pem: String = pem_public_key_from_rsa(&rsa);
        let node_id = read_peer_id_from_file();

        let amount_letters: String = encode(&amount_numbers);

        let mut drawee_peer_id = get_contact_from_map(&drawee_name);

        if drawee_peer_id.is_empty() {
            drawee_peer_id = drawee_name.clone();
        }

        let utc = Utc::now();
        let timestamp_at_drawing = utc.timestamp();
        let date_of_issue = utc.naive_local().date().to_string();
        let maturity_date = utc
            .checked_add_days(Days::new(BILL_VALIDITY_PERIOD))
            .unwrap()
            .naive_local()
            .date()
            .to_string();

        let new_bill = BitcreditBill {
            name: bill_name.clone(),
            to_payee: false,
            bill_jurisdiction,
            timestamp_at_drawing,
            place_of_drawing,
            currency_code: BTC.to_string(),
            amount_numbers,
            amounts_letters: amount_letters,
            maturity_date,
            date_of_issue,
            compounding_interest_rate: COMPOUNDING_INTEREST_RATE_ZERO,
            type_of_interest_calculation: false,
            place_of_payment: drawer.postal_address,
            public_key_pem,
            private_key_pem,
            language,
            drawee_name: drawee_peer_id,
            drawer_name: node_id.to_string(),
            holder_name: node_id.to_string(),
        };

        start_blockchain_for_new_bill(
            &new_bill,
            OperationCode::Issue,
            drawer.public_key_pem.clone(),
            drawer.private_key_pem.clone(),
        );

        new_bill
    }
}

fn create_bill_name(rsa: &Rsa<Private>) -> String {
    let public_key_bytes: Vec<u8> = rsa.public_key_to_pem().unwrap();
    let bill_name_hash: Vec<u8> = sha256(public_key_bytes.as_slice()).to_vec();
    let bill_name_readable = hex::encode(bill_name_hash);
    bill_name_readable
}

pub fn get_bills() -> Vec<BitcreditBill> {
    let mut bills = Vec::new();
    let paths = fs::read_dir(BILLS_FOLDER_PATH).unwrap();
    for _path in paths {
        let mut file_name = _path
            .unwrap()
            .file_name()
            .to_str()
            .expect("File name error")
            .to_string();
        //TODO change
        let path_without_extension = path::Path::file_stem(path::Path::new(&file_name))
            .expect("File name error")
            .to_str()
            .expect("File name error")
            .to_string();
        let bill = read_bill_from_file(&path_without_extension);
        bills.push(bill);
    }
    bills
}

pub fn endorse_bill_and_return_new_holder_id(bill_name: &String, new_holder: String) -> String {
    let contacts_map = read_contacts_map();

    let my_peer_id = read_peer_id_from_file().to_string();
    let mut bill = read_bill_from_file(&bill_name);

    let mut blockchain_from_file = Chain::read_chain_from_file(bill_name);
    let last_block = blockchain_from_file.get_latest_block();

    if my_peer_id.eq(&bill.holder_name) {
        if last_block.operation_code.eq(&OperationCode::Endorse)
            || last_block.operation_code.eq(&OperationCode::Issue)
            || last_block.operation_code.eq(&OperationCode::Accept)
            || last_block
                .operation_code
                .eq(&OperationCode::RequestToAccept)
        {
            let mut new_holder_node_id = "";
            if contacts_map.contains_key(&new_holder) {
                new_holder_node_id = contacts_map.get(&new_holder).expect("Contact not found");
            }
            if !new_holder_node_id.is_empty() {
                let identity = read_identity_from_file();

                bill.holder_name = new_holder_node_id.to_string().clone();

                let new_block = Block::new(
                    last_block.id + 1,
                    last_block.hash.clone(),
                    hex::encode(new_holder_node_id.clone().as_bytes()),
                    bill_name.clone(),
                    identity.public_key_pem.clone(),
                    OperationCode::Endorse,
                    identity.private_key_pem.clone(),
                );

                let try_add_block = blockchain_from_file.try_add_block(new_block.clone());
                if try_add_block && blockchain_from_file.is_chain_valid() {
                    blockchain_from_file.write_chain_to_file(&bill.name);
                    return new_holder_node_id.to_string().clone();
                } else {
                    return String::new();
                }
            } else {
                return String::new();
            }
        } else {
            return String::new();
        }
    } else {
        return String::new();
    }
}

pub fn request_pay(bill_name: &String) -> bool {
    let my_peer_id = read_peer_id_from_file().to_string();
    let bill = read_bill_from_file(bill_name);

    let mut blockchain_from_file = Chain::read_chain_from_file(bill_name);
    let last_block = blockchain_from_file.get_latest_block();

    let exist_code_with_accept =
        blockchain_from_file.exist_block_with_operation_code(OperationCode::Accept);
    let exist_code_with_request_to_accept =
        blockchain_from_file.exist_block_with_operation_code(OperationCode::RequestToAccept);
    let exist_code_with_request_to_pay =
        blockchain_from_file.exist_block_with_operation_code(OperationCode::RequestToPay);

    if exist_code_with_accept
        && exist_code_with_request_to_accept
        && !exist_code_with_request_to_pay
        && my_peer_id.eq(&bill.holder_name)
    {
        let identity = read_identity_from_file();

        let payable_info = generate_address_to_pay();

        if payable_info.2.is_spend_standard() {
            let new_block = Block::new(
                last_block.id + 1,
                last_block.hash.clone(),
                payable_info.2.to_string(),
                bill_name.clone(),
                identity.public_key_pem.clone(),
                OperationCode::Accept,
                identity.private_key_pem.clone(),
            );

            let try_add_block = blockchain_from_file.try_add_block(new_block.clone());
            if try_add_block && blockchain_from_file.is_chain_valid() {
                blockchain_from_file.write_chain_to_file(&bill.name);
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }
}

pub fn request_acceptance(bill_name: &String) -> bool {
    let my_peer_id = read_peer_id_from_file().to_string();
    let bill = read_bill_from_file(bill_name);

    let mut blockchain_from_file = Chain::read_chain_from_file(bill_name);
    let last_block = blockchain_from_file.get_latest_block();

    let exist_code_with_accept =
        blockchain_from_file.exist_block_with_operation_code(OperationCode::Accept);
    let exist_code_with_request_to_accept =
        blockchain_from_file.exist_block_with_operation_code(OperationCode::RequestToAccept);

    if !exist_code_with_accept
        && !exist_code_with_request_to_accept
        && my_peer_id.eq(&bill.holder_name)
    {
        if last_block.operation_code.eq(&OperationCode::Endorse)
            || last_block.operation_code.eq(&OperationCode::Issue)
        {
            let identity = read_identity_from_file();

            let new_block = Block::new(
                last_block.id + 1,
                last_block.hash.clone(),
                String::new(),
                bill_name.clone(),
                identity.public_key_pem.clone(),
                OperationCode::RequestToAccept,
                identity.private_key_pem.clone(),
            );

            let try_add_block = blockchain_from_file.try_add_block(new_block.clone());
            if try_add_block && blockchain_from_file.is_chain_valid() {
                blockchain_from_file.write_chain_to_file(&bill.name);
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }
}

pub fn accept_bill(bill_name: &String) -> bool {
    let bill = read_bill_from_file(bill_name);

    let mut blockchain_from_file = Chain::read_chain_from_file(bill_name);
    let last_block = blockchain_from_file.get_latest_block();

    let exist_code_with_accept =
        blockchain_from_file.exist_block_with_operation_code(OperationCode::Accept);
    let exist_code_with_request_to_accept =
        blockchain_from_file.exist_block_with_operation_code(OperationCode::RequestToAccept);

    if !exist_code_with_accept
        && exist_code_with_request_to_accept
    {
        let identity = read_identity_from_file();

        let new_block = Block::new(
            last_block.id + 1,
            last_block.hash.clone(),
            String::new(),
            bill_name.clone(),
            identity.public_key_pem.clone(),
            OperationCode::Accept,
            identity.private_key_pem.clone(),
        );

        let try_add_block = blockchain_from_file.try_add_block(new_block.clone());
        if try_add_block && blockchain_from_file.is_chain_valid() {
            blockchain_from_file.write_chain_to_file(&bill.name);
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn read_bill_from_file(bill_name: &String) -> BitcreditBill {
    let chain = Chain::read_chain_from_file(bill_name);
    let bill = chain.get_last_version_bill_with_operation_code(OperationCode::Endorse);
    bill
}

fn bill_to_byte_array(bill: &BitcreditBill) -> Vec<u8> {
    bill.try_to_vec().unwrap()
}

fn bill_from_byte_array(bill: &Vec<u8>) -> BitcreditBill {
    BitcreditBill::try_from_slice(bill).unwrap()
}
//--------------------------------------------------------------

//-------------------------Forms--------------------------------
#[derive(FromForm, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct BitcreditBillForm {
    pub bill_jurisdiction: String,
    pub place_of_drawing: String,
    pub amount_numbers: u64,
    pub language: String,
    pub drawee_name: String,
}

#[derive(FromForm, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EndorseBitcreditBillForm {
    pub new_holder: String,
    pub bill_name: String,
}

#[derive(FromForm, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RequestToAcceptBitcreditBillForm {
    pub bill_name: String,
}

#[derive(FromForm, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RequestToPayBitcreditBillForm {
    pub bill_name: String,
}

#[derive(FromForm, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AcceptBitcreditBillForm {
    pub bill_name: String,
    pub operation_code: String,
}

#[derive(FromForm, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct IdentityForm {
    name: String,
    date_of_birth: String,
    city_of_birth: String,
    country_of_birth: String,
    email: String,
    postal_address: String,
}

#[derive(FromForm, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewContactForm {
    pub name: String,
    pub node_id: String,
}
//-------------------------------------------------------------
