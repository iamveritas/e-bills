#[macro_use]
extern crate rocket;

mod constants;
mod dht;
mod numbers_to_words;
mod test;
mod web;

use crate::constants::{
    BILLS_FOLDER_PATH, BILL_VALIDITY_PERIOD, BTC, COMPOUNDING_INTEREST_RATE_ZERO, DHT_FILE_PATH,
    DHT_FOLDER_PATH, IDENTITY_ED_25529_KEYS_FILE_PATH, IDENTITY_FILE_PATH, IDENTITY_FOLDER_PATH,
    IDENTITY_PEER_ID_FILE_PATH,
};
use crate::numbers_to_words::encode;

use borsh::{self, BorshDeserialize, BorshSerialize};
use chrono::{Days, Utc};
use libp2p::identity::Keypair;
use libp2p::kad::store::MemoryStore;
use libp2p::kad::Kademlia;
use libp2p::PeerId;
use openssl::pkey::{Private, Public};
use openssl::rsa;
use openssl::rsa::{Padding, Rsa};
use openssl::sha::sha256;
use rocket::fs::FileServer;
use rocket::serde::{Deserialize, Serialize};
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;
use std::path::Path;
use std::{fs, mem};

// MAIN
#[rocket::main]
async fn main() {
    let mut dht = dht::dht_main().await.unwrap();

    if Path::new(IDENTITY_PEER_ID_FILE_PATH).exists() {
        let local_peer_id = read_peer_id_from_file();
        dht.check_new_bills_when_login(local_peer_id.to_string()).await;
    }

    let rocket = rocket_main(dht).launch().await.unwrap();

    //TODO: how to stay program online without it.
    loop {}
}

fn rocket_main(dht: dht::network::Client) -> Rocket<Build> {
    rocket::build()
        .register("/", catchers![web::not_found])
        .manage(dht)
        .mount("/image", FileServer::from("image"))
        .mount("/css", FileServer::from("css"))
        .mount("/", routes![web::start])
        .mount(
            "/identity",
            routes![web::get_identity, web::create_identity,],
        )
        .mount("/bills", routes![web::bills_list])
        .mount("/info", routes![web::info])
        .mount(
            "/bill",
            routes![
                web::get_bill,
                web::issue_bill,
                web::new_bill,
                web::search_bill_dht,
                web::search_bill
            ],
        )
        .attach(Template::custom(|engines| {
            web::customize(&mut engines.handlebars);
        }))
}

// CORE

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

// IDENITY

pub struct IdentityWithAll {
    identity: Identity,
    peer_id: PeerId,
    key_pair: Keypair,
}

// Private individuals or legal entities.
#[derive(BorshSerialize, BorshDeserialize, FromForm, Debug, Serialize, Deserialize)]
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
    if !Path::new(IDENTITY_FOLDER_PATH).exists() {
        let identity = create_new_identity(
            name,
            date_of_birth,
            city_of_birth,
            country_of_birth,
            email,
            postal_address,
        );
        let ed25519_keys: Keypair = Keypair::generate_ed25519();
        let peer_id: PeerId = ed25519_keys.public().to_peer_id();

        write_peer_id_to_file(&peer_id);
        write_ed25519_keypair_to_file(&ed25519_keys);
        write_identity_to_file(&identity);

        IdentityWithAll {
            identity,
            peer_id,
            key_pair: ed25519_keys,
        }
    } else {
        let identity: Identity = read_identity_from_file();
        let ed25519_keys: Keypair = read_ed25519_keypair_from_file();
        let peer_id: PeerId = read_peer_id_from_file();

        IdentityWithAll {
            identity,
            peer_id,
            key_pair: ed25519_keys,
        }
    }
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

    if !Path::new(IDENTITY_FOLDER_PATH).exists() {
        fs::create_dir(IDENTITY_FOLDER_PATH).expect("Can't create folder identity.");
    }

    fs::write(IDENTITY_FILE_PATH, data).expect("Unable to write file identity");
}

fn write_ed25519_keypair_to_file(ed25519_keys: &Keypair) {
    let data: &[u8] = unsafe { structure_as_u8_slice(ed25519_keys) };
    let data_sized = byte_array_to_size_array_keypair(data);

    if !Path::new(IDENTITY_FOLDER_PATH).exists() {
        fs::create_dir(IDENTITY_FOLDER_PATH).expect("Can't create folder ed25519 keypair");
    }

    fs::write(IDENTITY_ED_25529_KEYS_FILE_PATH, *data_sized)
        .expect("Unable to write keypair ed25519 in file");
}

fn write_peer_id_to_file(peer_id: &PeerId) {
    let data: &[u8] = unsafe { structure_as_u8_slice(peer_id) };
    let data_sized = byte_array_to_size_array_peer_id(data);

    if !Path::new(IDENTITY_FOLDER_PATH).exists() {
        fs::create_dir(IDENTITY_FOLDER_PATH).expect("Can't create folder peer id");
    }

    fs::write(IDENTITY_PEER_ID_FILE_PATH, *data_sized).expect("Unable to write peer id in file");
}

fn write_dht_to_file(dht: &Kademlia<MemoryStore>) {
    let data: &[u8] = unsafe { structure_as_u8_slice(dht) };
    let data_sized = byte_array_to_size_array_dht(data);

    if !Path::new(DHT_FOLDER_PATH).exists() {
        fs::create_dir(DHT_FOLDER_PATH).expect("Can't create folder peer id");
    }

    fs::write(DHT_FILE_PATH, *data_sized).expect("Unable to write peer id in file");
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

fn read_dht_from_file() -> Kademlia<MemoryStore> {
    let data: Vec<u8> = fs::read(DHT_FILE_PATH).expect("Unable to read file with dht");
    let dht_bytes_sized = byte_array_to_size_array_dht(data.as_slice());
    let dht: Kademlia<MemoryStore> = unsafe { mem::transmute_copy(dht_bytes_sized) };
    dht
}

fn identity_to_byte_array(identity: &Identity) -> Vec<u8> {
    identity.try_to_vec().unwrap()
}

fn identity_from_byte_array(identity: &Vec<u8>) -> Identity {
    Identity::try_from_slice(identity).unwrap()
}

fn byte_array_to_size_array_dht(
    array: &[u8],
) -> &[u8; ::std::mem::size_of::<Kademlia<MemoryStore>>()] {
    array.try_into().expect("slice with incorrect length")
}

fn byte_array_to_size_array_keypair(array: &[u8]) -> &[u8; ::std::mem::size_of::<Keypair>()] {
    array.try_into().expect("slice with incorrect length")
}

fn byte_array_to_size_array_peer_id(array: &[u8]) -> &[u8; ::std::mem::size_of::<PeerId>()] {
    array.try_into().expect("slice with incorrect length")
}

// BILL

// A cryptographic bill of exchange with future repayment.
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
    // Default - the drawer’s address.
    place_of_drawing: String,
    // In MVP only BTC.
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
        let private_key: String = pem_private_key_from_rsa(&rsa);
        let public_key: String = pem_public_key_from_rsa(&rsa);

        let amount_letters: String = encode(&amount_numbers);

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
            name: bill_name,
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
            public_key_pem: public_key,
            private_key_pem: private_key,
            language,
            drawee_name,
            drawer_name: drawer.name,
        };

        write_bill_to_file(&new_bill);

        new_bill
    }
}

fn create_bill_name(rsa: &Rsa<Private>) -> String {
    let public_key_bytes: Vec<u8> = rsa.public_key_to_pem().unwrap();
    let bill_name_hash: Vec<u8> = sha256(public_key_bytes.as_slice()).to_vec();
    let bill_name_hash: String = format!("{:?}", &bill_name_hash);
    let bill_name: String = clear_bill_name(bill_name_hash);

    bill_name
}

fn clear_bill_name(bill_name_hash: String) -> String {
    let bill_name: String = bill_name_hash.replace(", ", "").replace(['[', ']'], "");

    bill_name
}

fn write_bill_to_file(bill: &BitcreditBill) {
    let data: Vec<u8> = bill_to_byte_array(bill);
    if !Path::new(BILLS_FOLDER_PATH).exists() {
        fs::create_dir(BILLS_FOLDER_PATH).expect("Can't create folder bills");
    }
    let path: String = BILLS_FOLDER_PATH.to_string() + "/" + &bill.name;
    fs::write(path.as_str(), data).expect("Unable to write bill file");
}

fn read_bill_from_file(bill_id: &String) -> BitcreditBill {
    let path: String = BILLS_FOLDER_PATH.to_string() + "/" + bill_id;
    let data: Vec<u8> = fs::read(path.as_str()).expect("Unable to read file bill");
    bill_from_byte_array(&data)
}

fn bill_to_byte_array(bill: &BitcreditBill) -> Vec<u8> {
    bill.try_to_vec().unwrap()
}

fn bill_from_byte_array(bill: &Vec<u8>) -> BitcreditBill {
    BitcreditBill::try_from_slice(bill).unwrap()
}

//FORMS

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
pub struct FindBillForm {
    pub bill_name: String,
}
