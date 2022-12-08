mod numbers_to_words;

use borsh::{self, BorshDeserialize, BorshSerialize};
use openssl::pkey::Private;
use openssl::rsa::Rsa;
use std::mem;

// Private individuals or legal entities.
pub struct Identity {
    //TODO: cryptographic address.

    //TODO: add hash of all data.

    // Name + surname or company name.
    legal_name: String,

    // Date of birth or foundation in the format YYYY-MM-DD.
    // TODO: change to data type.
    date_of_appearance: String,

    // City of birth or foundation.
    city_of_appearance: String,

    // Country of birth or foundation.
    country_of_appearance: String,

    email: String,

    // Current address (living or registration).
    current_address: String,

    // Central liability provider Public Key.
    liability_provider: String,

    public_key_pem: String,

    private_key_pem: String,
}

pub fn create_new_identity(
    legal_name: String,
    date_of_appearance: String,
    city_of_appearance: String,
    country_of_appearance: String,
    email: String,
    current_address: String,
) -> Identity {
    let rsa = generation_rsa_key();

    let private_key = pem_private_key_from_rsa(&rsa);

    let public_key = pem_public_key_from_rsa(&rsa);

    let new_identity = Identity {
        legal_name: legal_name,
        date_of_appearance: date_of_appearance,
        city_of_appearance: city_of_appearance,
        country_of_appearance: country_of_appearance,
        email: email,
        current_address: current_address,
        liability_provider: "".to_string(),
        public_key_pem: public_key,
        private_key_pem: private_key,
    };

    new_identity
}

fn generation_rsa_key() -> Rsa<Private> {
    Rsa::generate(2048).unwrap()
}

fn pem_private_key_from_rsa(rsa: &Rsa<Private>) -> String {
    let private_key: Vec<u8> = rsa.private_key_to_pem().unwrap();
    //TODO: Write it in file
    String::from_utf8(private_key).unwrap()
}

fn pem_public_key_from_rsa(rsa: &Rsa<Private>) -> String {
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();
    //TODO: Write it in file
    String::from_utf8(public_key).unwrap()
}

// A cryptographic bill of exchange with future repayment.
#[derive(BorshSerialize, BorshDeserialize)]
pub struct BitcreditBill {
    // Flag: “to” the payee or “to his order”.
    to_payee: bool,

    bill_jurisdiction: String,

    //TODO: change to normal timestamp
    timestamp_at_drawing: String,

    // Default - the drawer’s address.
    place_of_drawing: String,

    // In MVP only BTC.
    currency_code: String,

    // Formatted for country.
    amount_numbers: u64,

    amounts_letters: String,

    // TODO: change to data type.
    maturity_date: String,

    // Percent of interest.
    compounding_interest_rate: u64,

    // A flag for interest calculation “in advance” (for bills) or “in arrears” (for notes).
    type_of_interest_calculation: bool,

    //TODO: each of the three party’s id / (optionally encrypted with the bill’s public key).

    //TODO: each name (drawer, drawee, payee) and their respective address (defaulting to the id/ data), always encrypted with the bill’s public key.

    // Defaulting to the drawee’s id/ address, encrypted with the bill’s public key.
    place_of_payment: String,

    public_key_pem: String,

    private_key_pem: String,

    // In MVP english or german.
    language: String,
}

pub fn issue_new_bill(
    bill_jurisdiction: String,
    place_of_drawing: String,
    amount_numbers: u64,
    amounts_letters: String,
    maturity_date: String,
    drawee: Identity,
    language: String,
) -> BitcreditBill {
    // Generate keys.
    let rsa = generation_rsa_key();

    let private_key = pem_private_key_from_rsa(&rsa);

    let public_key = pem_public_key_from_rsa(&rsa);

    // Create bill.
    let new_bill = BitcreditBill {
        to_payee: false,
        bill_jurisdiction: bill_jurisdiction,
        timestamp_at_drawing: "test".to_string(),
        place_of_drawing: place_of_drawing,
        currency_code: "BTC".to_string(),
        amount_numbers: amount_numbers,
        amounts_letters: amounts_letters,
        maturity_date: maturity_date,
        compounding_interest_rate: 0,
        type_of_interest_calculation: false,
        place_of_payment: drawee.current_address.to_string(),
        public_key_pem: public_key,
        private_key_pem: private_key,
        language: language,
    };

    // Return new bill.
    new_bill
}

fn encrypt_bill() {}

fn decrypt_bill() {}

// The party issuing a bill.
struct Drawer {}

// The party obliged to pay a bill.
struct Drawee {}

// The person to whom a bill is payable.
struct Payee {}

// The person to whom the payee or a prior holder endorses a bill.
struct Holder {}

// For the issuer of a dishonored bill or note, all issued bills and notes become due immediately.
struct Honor {}

fn main() {
    // Add new identity in our DHT.
}

#[cfg(test)]
mod test {
    use crate::numbers_to_words::encode;
    use crate::{
        create_new_identity, generation_rsa_key, issue_new_bill, pem_private_key_from_rsa,
        pem_public_key_from_rsa, BitcreditBill, Identity,
    };
    use borsh::{BorshDeserialize, BorshSerialize};
    use openssl::aes::{aes_ige, AesKey};
    use openssl::encrypt::{Decrypter, Encrypter};
    use openssl::hash::MessageDigest;
    use openssl::pkey::{PKey, PKeyRef};
    use openssl::rand::rand_bytes;
    use openssl::rsa::{Padding, Rsa};
    use openssl::sign::{Signer, Verifier};
    use openssl::symm::{decrypt, encrypt, Cipher, Mode};

    #[test]
    fn different_rsa_keys_generated_each_time() {
        let key1 = generation_rsa_key();
        let key2 = generation_rsa_key();
        assert_ne!(
            key1.private_key_to_pem().unwrap(),
            key2.private_key_to_pem().unwrap(),
            "private key from 2 generation must be NOT equal"
        );
        assert_ne!(
            key1.public_key_to_pem().unwrap(),
            key2.public_key_to_pem().unwrap(),
            "public key from 2 generation must be NOT equal"
        );
    }

    #[test]
    fn sign_and_verify_data_given_an_rsa_keypair() {
        // Create data
        let data: BitcreditBill = issue_new_bill(
            "".to_string(),
            "".to_string(),
            0,
            "".to_string(),
            "".to_string(),
            Identity {
                legal_name: "".to_string(),
                date_of_appearance: "".to_string(),
                city_of_appearance: "".to_string(),
                country_of_appearance: "".to_string(),
                email: "".to_string(),
                current_address: "".to_string(),
                liability_provider: "".to_string(),
                public_key_pem: "".to_string(),
                private_key_pem: "".to_string(),
            },
            "".to_string(),
        );

        // Generate a keypair
        let rsa_key = generation_rsa_key();
        let p_key = PKey::from_rsa(rsa_key).unwrap();

        // Create signer
        let mut signer = Signer::new(MessageDigest::sha256(), p_key.as_ref()).unwrap();

        // Sign
        signer.update(&*data.try_to_vec().unwrap()).unwrap();
        let signature = signer.sign_to_vec().unwrap();

        // Create verifier
        let mut verifier = Verifier::new(MessageDigest::sha256(), p_key.as_ref()).unwrap();

        // Verify
        verifier.update(&*data.try_to_vec().unwrap()).unwrap();
        assert!(verifier.verify(&signature).unwrap());
    }

    #[test]
    fn encrypt_and_decrypt_simple_data_given_an_rsa_keypair_using_encrypter_decrypter() {
        // Create data
        let data = "test";

        // Generate a keypair
        let rsa_key = generation_rsa_key();
        let p_key = PKey::from_rsa(rsa_key).unwrap();

        // Encrypt the data with RSA PKCS1
        let mut encrypter = Encrypter::new(&p_key).unwrap();
        encrypter.set_rsa_padding(Padding::PKCS1).unwrap();
        // Create an output buffer
        let buffer_len = encrypter.encrypt_len(data.as_bytes()).unwrap();
        let mut encrypted = vec![0; buffer_len];
        // Encrypt and truncate the buffer
        let encrypted_len = encrypter.encrypt(data.as_bytes(), &mut encrypted).unwrap();
        encrypted.truncate(encrypted_len);

        // Decrypt the data
        let mut decrypter = Decrypter::new(&p_key).unwrap();
        decrypter.set_rsa_padding(Padding::PKCS1).unwrap();
        // Create an output buffer
        let buffer_len = decrypter.decrypt_len(&encrypted).unwrap();
        let mut decrypted = vec![0; buffer_len];
        // Encrypt and truncate the buffer
        let decrypted_len = decrypter.decrypt(&encrypted, &mut decrypted).unwrap();
        decrypted.truncate(decrypted_len);

        assert_eq!(&*decrypted, data.as_bytes());
    }

    #[test]
    fn encrypt_and_decrypt_simple_data_with_rsa_keypair() {
        // Create data
        let data = "test";

        // Generate a keypair
        let rsa_key = generation_rsa_key();

        // Encrypt with public key
        let mut buf: Vec<u8> = vec![0; rsa_key.size() as usize];
        let _ = rsa_key
            .public_encrypt(data.as_bytes(), &mut buf, Padding::PKCS1)
            .unwrap();

        let data_enc = buf;

        // Decrypt with private key
        let mut buf: Vec<u8> = vec![0; rsa_key.size() as usize];
        let _ = rsa_key
            .private_decrypt(&data_enc, &mut buf, Padding::PKCS1)
            .unwrap();
        assert!(String::from_utf8(buf).unwrap().starts_with(data));
    }

    // #[test]
    // fn create_bill() {
    //     // Create bill
    //     let bill = issue_new_bill();
    //     let bill_public_key = &bill.public_key_pem;
    //     let bill_private_key = &bill.private_ket_encrypted_with_pass_pem;
    //
    //     // Encrypted bill with public key
    //     let rsa_bill = Rsa::public_key_from_pem(bill_public_key.as_bytes()).unwrap();
    //     let mut buf_bill: Vec<u8> = vec![0; rsa_bill.size() as usize];
    //     let bill_byte = bill.try_to_vec().unwrap();
    //     let _ = rsa_bill.public_encrypt(bill_byte.as_slice(), &mut buf_bill, Padding::PKCS1).unwrap();
    //     let encrypted_bill = buf_bill;
    //
    //     // Create 3 person
    //     let payee = create_new_id();
    //     let payee_public_key = &payee.public_key_pem;
    //     let payee_private_key = &payee.private_ket_encrypted_with_pass_pem;
    //     let drawer = create_new_id();
    //     let drawer_public_key = &drawer.public_key_pem;
    //     let drawer_private_key = &drawer.private_ket_encrypted_with_pass_pem;
    //     let drawee = create_new_id();
    //     let drawee_public_key = &drawee.public_key_pem;
    //     let drawee_private_key = &drawee.private_ket_encrypted_with_pass_pem;
    //
    //     // Encrypted bill's private key with person's public keys
    //     let bill_private_key_byte = bill_private_key.as_bytes();
    //     // payee
    //     let rsa_payee = Rsa::public_key_from_pem(payee_public_key.as_bytes()).unwrap();
    //     let mut buf_payee: Vec<u8> = vec![0; rsa_payee.size() as usize];
    //     let _ = rsa_payee.public_encrypt(bill_private_key_byte, &mut buf_payee, Padding::PKCS1).unwrap();
    //     let encrypted_bill_private_key_payee = buf_payee;
    //     // drawer
    //     let rsa_drawer = Rsa::public_key_from_pem(drawer_public_key.as_bytes()).unwrap();
    //     let mut buf_drawer: Vec<u8> = vec![0; rsa_drawer.size() as usize];
    //     let _ = rsa_drawer.public_encrypt(bill_private_key_byte, &mut buf_drawer, Padding::PKCS1).unwrap();
    //     let encrypted_bill_private_key_drawer = buf_drawer;
    //     // drawee
    //     let rsa_drawee = Rsa::public_key_from_pem(drawee_public_key.as_bytes()).unwrap();
    //     let mut buf_drawee: Vec<u8> = vec![0; rsa_drawee.size() as usize];
    //     let _ = rsa_drawee.public_encrypt(bill_private_key_byte, &mut buf_drawee, Padding::PKCS1).unwrap();
    //     let encrypted_bill_private_key_drawee = buf_drawee;
    //
    //     // Decrypted bill's private key with person's private keys
    //     let passphrase = "Qwerty1234";
    //     // payee
    //     let rsa_payee_pr = Rsa::private_key_from_pem_passphrase(payee_private_key.as_bytes(), passphrase.as_bytes()).unwrap();
    //     let mut buf_payee_pr: Vec<u8> = vec![0; rsa_payee_pr.size() as usize];
    //     let _ = rsa_payee_pr.private_decrypt(&encrypted_bill_private_key_payee, &mut buf_payee_pr, Padding::PKCS1).unwrap();
    //     let payee_key_from_bill = String::from_utf8(buf_payee_pr).unwrap();
    //     // drawer
    //     let rsa_drawer_pr = Rsa::private_key_from_pem_passphrase(drawer_private_key.as_bytes(), passphrase.as_bytes()).unwrap();
    //     let mut buf_drawer_pr: Vec<u8> = vec![0; rsa_drawer_pr.size() as usize];
    //     let _ = rsa_drawer_pr.private_decrypt(&encrypted_bill_private_key_drawer, &mut buf_drawer_pr, Padding::PKCS1).unwrap();
    //     // drawee
    //     let rsa_drawee_pr = Rsa::private_key_from_pem_passphrase(drawee_private_key.as_bytes(), passphrase.as_bytes()).unwrap();
    //     let mut buf_drawee_pr: Vec<u8> = vec![0; rsa_drawee_pr.size() as usize];
    //     let _ = rsa_drawee_pr.private_decrypt(&encrypted_bill_private_key_drawee, &mut buf_drawee_pr, Padding::PKCS1).unwrap();
    //
    //     // Decrypted bill with private key
    //
    // }
    //
    // #[test]
    // unsafe fn test () {
    //     let passphrase = "Qwerty1234";
    //
    //     let bill = issue_new_bill();
    //     let bill_public_key = &bill.public_key_pem;
    //     let bill_private_key = &bill.private_ket_encrypted_with_pass_pem;
    //
    //     //
    //     let mut buf = [0; 128];
    //     rand_bytes(&mut buf).unwrap();
    //     let aeskey = AesKey::new_encrypt(&buf).unwrap();
    //
    //     let payee = create_new_id();
    //     let payee_public_key = &payee.public_key_pem;
    //     let payee_private_key = &payee.private_ket_encrypted_with_pass_pem;
    //
    //
    //     // // Encrypt with public key
    //     // let rsa = Rsa::public_key_from_pem(payee_public_key.as_bytes()).unwrap();
    //     // let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    //     // let _ = rsa.public_encrypt(bill_private_key.as_bytes(), &mut buf, Padding::PKCS1).unwrap();
    //     //
    //     // let data = buf;
    //     //
    //     // // Decrypt with private key
    //     // let rsa = Rsa::private_key_from_pem_passphrase(payee_private_key.as_bytes(), passphrase.as_bytes()).unwrap();
    //     // let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    //     // let _ = rsa.private_decrypt(&data, &mut buf, Padding::PKCS1).unwrap();
    // }

    #[test]
    fn encrypting_assymetric_rsa_key_with_symmetric_cipher() {
        let cipher = Cipher::aes_128_cbc();
        let data = b"Some Crypto Text";
        let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
        let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

        let ciphertext_enc = encrypt(cipher, key, Some(iv), data).unwrap();

        assert_eq!(
            b"\xB4\xB9\xE7\x30\xD6\xD6\xF7\xDE\x77\x3F\x1C\xFF\xB3\x3E\x44\x5A\x91\xD7\x27\x62\x87\x4D\xFB\x3C\x5E\xC4\x59\x72\x4A\xF4\x7C\xA1",
            &ciphertext_enc[..]);

        let ciphertext = decrypt(cipher, key, Some(iv), &ciphertext_enc[..]).unwrap();

        assert_eq!(b"Some Crypto Text", &ciphertext[..]);
    }

    #[test]
    fn bill_to_bytes_and_opposite_with_borsh() {
        let bill = issue_new_bill(
            "".to_string(),
            "".to_string(),
            0,
            "".to_string(),
            "".to_string(),
            Identity {
                legal_name: "".to_string(),
                date_of_appearance: "".to_string(),
                city_of_appearance: "".to_string(),
                country_of_appearance: "".to_string(),
                email: "".to_string(),
                current_address: "".to_string(),
                liability_provider: "".to_string(),
                public_key_pem: "".to_string(),
                private_key_pem: "".to_string(),
            },
            "".to_string(),
        );

        let encoded_bill = bill.try_to_vec().unwrap();

        let decoded_a: BitcreditBill = BitcreditBill::try_from_slice(&encoded_bill).unwrap();
        assert_eq!(bill.bill_jurisdiction, decoded_a.bill_jurisdiction);
    }

    #[test]
    fn create_new_account() {
        let account: Identity = create_new_identity(
            "Ivan".to_string(),
            "12.12.2022".to_string(),
            "Vienna".to_string(),
            "Austria".to_string(),
            "111@gmail.com".to_string(),
            "Mainstrasse 1".to_string(),
        );
    }

    #[test]
    fn numbers_to_letters() {
        let result = encode(123_324_324);
        assert_eq!("one hundred twenty-three million three hundred twenty-four thousand three hundred twenty-four".to_string(), result);
    }
}
