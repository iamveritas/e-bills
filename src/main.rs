mod numbers_to_words;

use borsh::{self, BorshDeserialize, BorshSerialize};
use openssl::pkey::Private;
use openssl::rsa::{Padding, Rsa};
use std::fs::File;
use std::{fs, mem};

const STORAGE_FILE_PATH: &str = "./identity";

// Private individuals or legal entities.
pub struct Identity {
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
    String::from_utf8(private_key).unwrap()
}

fn pem_public_key_from_rsa(rsa: &Rsa<Private>) -> String {
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();
    String::from_utf8(public_key).unwrap()
}

// A cryptographic bill of exchange with future repayment.
#[derive(BorshSerialize, BorshDeserialize)]
pub struct BitcreditBill {
    id: i32,

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

    maturity_date: String,

    // Percent of interest.
    compounding_interest_rate: u64,

    // A flag for interest calculation “in advance” (for bills) or “in arrears” (for notes).
    type_of_interest_calculation: bool,

    // Defaulting to the drawee’s id/ address, encrypted with the bill’s public key.
    place_of_payment: String,

    public_key_pem: String,

    private_key_pem: String,

    // In MVP english or german.
    language: String,
}

pub fn issue_new_bill(
    id: i32,
    bill_jurisdiction: String,
    place_of_drawing: String,
    amount_numbers: u64,
    amounts_letters: String,
    maturity_date: String,
    drawee: Identity,
    language: String,
) -> BitcreditBill {
    let rsa = generation_rsa_key();

    let private_key = pem_private_key_from_rsa(&rsa);

    let public_key = pem_public_key_from_rsa(&rsa);

    let new_bill = BitcreditBill {
        id: id,
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

    new_bill
}

fn write_bill_to_file(bill: &BitcreditBill) {
    // late it need to choose name of file with bill
    let bill_id = bill.id;
    let data = bill_to_byte_array(&bill);
    fs::write("bills/test", data).expect("Unable to write file");
}

fn read_bill_from_file(bill_id: &i32) -> BitcreditBill {
    let data = fs::read("bills/test").expect("Unable to read file");
    bill_from_byte_array(&data)
}

fn bill_to_byte_array(bill: &BitcreditBill) -> Vec<u8> {
    bill.try_to_vec().unwrap()
}

fn bill_from_byte_array(bill: &Vec<u8>) -> BitcreditBill {
    BitcreditBill::try_from_slice(&bill).unwrap()
}

fn encrypt_bytes(bill_bytes: &Vec<u8>, rsa_key: &Rsa<Private>) -> Vec<u8> {
    let key_size = (rsa_key.size() / 2) as usize; //128

    let mut hole_encrypted_buff = Vec::new();
    let mut time_buff = vec![0; key_size];
    let mut time_buff_encrypted = vec![0; rsa_key.size() as usize];

    let number_of_key_size_in_hole_bill = bill_bytes.len() / key_size;
    let remainder = bill_bytes.len() - key_size * number_of_key_size_in_hole_bill;

    for i in 0..number_of_key_size_in_hole_bill {
        for j in 0..key_size {
            let byte_number = key_size * i + j;
            time_buff[j] = bill_bytes[byte_number];
        }

        let encrypted_len = rsa_key
            .public_encrypt(&*time_buff, &mut time_buff_encrypted, Padding::PKCS1)
            .unwrap();

        hole_encrypted_buff.append(&mut time_buff_encrypted);
        time_buff = vec![0; key_size];
        time_buff_encrypted = vec![0; rsa_key.size() as usize];
    }

    if remainder != 0 {
        time_buff = vec![0; remainder];

        let position = key_size * number_of_key_size_in_hole_bill;
        let mut index_in_time_buff = 0;

        for i in position..bill_bytes.len() {
            time_buff[index_in_time_buff] = bill_bytes[i];
            index_in_time_buff += 1;
        }

        index_in_time_buff = 0;

        let encrypted_len = rsa_key
            .public_encrypt(&*time_buff, &mut time_buff_encrypted, Padding::PKCS1)
            .unwrap();

        hole_encrypted_buff.append(&mut time_buff_encrypted);
        time_buff.clear();
        time_buff_encrypted.clear();
    }

    hole_encrypted_buff
}

fn decrypt_bytes(bill_bytes: &Vec<u8>, rsa_key: &Rsa<Private>) -> Vec<u8> {
    let key_size = rsa_key.size() as usize; //256

    let mut hole_decrypted_buff = Vec::new();
    let mut time_buff = vec![0; rsa_key.size() as usize];
    let mut time_buff_decrypted = vec![0; rsa_key.size() as usize];

    let number_of_key_size_in_hole_bill = bill_bytes.len() / key_size;
    // let remainder = bill_bytes.len() - key_size * number_of_key_size_in_hole_bill;

    for i in 0..number_of_key_size_in_hole_bill {
        for j in 0..key_size {
            let byte_number = key_size * i + j;
            time_buff[j] = bill_bytes[byte_number];
        }

        let decrypted_len = rsa_key
            .private_decrypt(&*time_buff, &mut time_buff_decrypted, Padding::PKCS1)
            .unwrap();

        hole_decrypted_buff.append(&mut time_buff_decrypted[0..decrypted_len].to_vec());
        time_buff = vec![0; rsa_key.size() as usize];
        time_buff_decrypted = vec![0; rsa_key.size() as usize];
    }

    // if remainder != 0 {
    //     let position = key_size * number_of_key_size_in_hole_bill;
    //     let mut index_in_time_buff = 0;
    //
    //     for i in position..bill_bytes.len() {
    //         time_buff[index_in_time_buff] = bill_bytes[i];
    //         index_in_time_buff = index_in_time_buff + 1;
    //     }
    //
    //     index_in_time_buff = 0;
    //
    //     let decrypted_len = rsa_key
    //         .public_decrypt(&*time_buff, &mut time_buff_decrypted, Padding::PKCS1)
    //         .unwrap();
    //
    //     hole_decrypted_buff.append(&mut time_buff_decrypted);
    //     time_buff.clear();
    //     time_buff_decrypted.clear();
    // }

    hole_decrypted_buff
}

unsafe fn any_as_u8_slice<'a, T: Sized>(mut p: T) -> &'a mut [u8] {
    ::std::slice::from_raw_parts_mut((&mut p as *mut T) as *mut u8, ::std::mem::size_of::<T>())
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::numbers_to_words::encode;
    use crate::{
        any_as_u8_slice, bill_from_byte_array, bill_to_byte_array, create_new_identity,
        decrypt_bytes, encrypt_bytes, generation_rsa_key, issue_new_bill, pem_private_key_from_rsa,
        pem_public_key_from_rsa, read_bill_from_file, write_bill_to_file, BitcreditBill, Identity,
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
    use std::io::Read;
    use std::{env, fs, slice};

    // Dont uncomment before find way for special file name for every bill.
    // #[test]
    // fn write_bill_to_file_and_read_it() {
    //     let bill = issue_new_bill(
    //         1,
    //         "fa".to_string(),
    //         "saf".to_string(),
    //         23,
    //         "sdf".to_string(),
    //         "11".to_string(),
    //         Identity {
    //             legal_name: "tymko".to_string(),
    //             date_of_appearance: "2123".to_string(),
    //             city_of_appearance: "13".to_string(),
    //             country_of_appearance: "123".to_string(),
    //             email: "123".to_string(),
    //             current_address: "123".to_string(),
    //             liability_provider: "123".to_string(),
    //             public_key_pem: "123".to_string(),
    //             private_key_pem: "321".to_string(),
    //         },
    //         "3213".to_string(),
    //     );
    //
    //     write_bill_to_file(bill);
    //     let bill_from_file = read_bill_from_file(1);
    //
    //     assert_eq!("fa".to_string(), bill_from_file.bill_jurisdiction);
    // }

    #[test]
    fn encrypt_bill_with_rsa_keypair() {
        let bill = issue_new_bill(
            0,
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

        let rsa_key = generation_rsa_key();
        let bill_bytes = bill_to_byte_array(&bill);

        let enc = encrypt_bytes(&bill_bytes, &rsa_key);

        let mut final_number_256_byte_arrays: u32;
        let bill_bytes_len = bill_bytes.len();
        let exact_number_256_byte_arrays = (bill_bytes_len as f32 / 128 as f32) as f32;
        if exact_number_256_byte_arrays % 1.0 == 0 as f32 {
            final_number_256_byte_arrays = exact_number_256_byte_arrays as u32
        } else {
            final_number_256_byte_arrays = exact_number_256_byte_arrays as u32 + 1
        }

        assert_eq!(final_number_256_byte_arrays * 256, enc.len() as u32);
    }

    #[test]
    fn decrypt_bill_with_rsa_keypair() {
        let bill = issue_new_bill(
            0,
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

        let rsa_key = generation_rsa_key();
        let bill_bytes = bill_to_byte_array(&bill);

        let encrypted_bill = encrypt_bytes(&bill_bytes, &rsa_key);

        let decrypted_bill = decrypt_bytes(&encrypted_bill, &rsa_key);
        assert_eq!(bill_bytes.len(), decrypted_bill.len());

        let new_bill = bill_from_byte_array(&decrypted_bill);

        assert_eq!(bill.bill_jurisdiction, new_bill.bill_jurisdiction);
    }

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
            0,
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

    #[test]
    fn encrypting_asymmetric_rosa_key_with_symmetric_cipher() {
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
            0,
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

        let decoded_bill: BitcreditBill = BitcreditBill::try_from_slice(&encoded_bill).unwrap();
        assert_eq!(bill.bill_jurisdiction, decoded_bill.bill_jurisdiction);
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
