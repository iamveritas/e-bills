#[cfg(test)]
mod test {
    use std::io::Read;
    use std::path::Path;
    use std::time::Duration;
    use std::{fs, mem};

    use borsh::{BorshDeserialize, BorshSerialize};
    use libp2p::identity::Keypair;
    use libp2p::kad::store::MemoryStore;
    use libp2p::kad::{Kademlia, KademliaConfig};
    use libp2p::{identity, PeerId};
    use openssl::hash::MessageDigest;
    use openssl::pkey::{PKey, Private, Public};
    use openssl::rsa::{Padding, Rsa};
    use openssl::sha::sha256;
    use openssl::sign::{Signer, Verifier};
    use openssl::symm::Cipher;
    use openssl::{aes, sha};

    use crate::numbers_to_words::encode;
    use crate::{
        bill_from_byte_array, bill_to_byte_array, byte_array_to_size_array_keypair,
        byte_array_to_size_array_peer_id, create_new_identity, decrypt_bytes, encrypt_bytes,
        generation_rsa_key, issue_new_bill, pem_private_key_from_rsa, pem_public_key_from_rsa,
        private_key_from_pem_u8, public_key_from_pem_u8, read_bill_from_file,
        structure_as_u8_slice, write_bill_to_file, BitcreditBill, Identity,
    };

    #[test]
    fn write_bill_to_file_and_read_it() {
        let bill = issue_new_bill(
            "fa".to_string(),
            "".to_string(),
            0,
            Identity {
                name: "".to_string(),
                date_of_birth: "".to_string(),
                city_of_birth: "".to_string(),
                country_of_birth: "".to_string(),
                email: "".to_string(),
                postal_address: "".to_string(),
                public_key_pem: "".to_string(),
                private_key_pem: "".to_string(),
            },
            "".to_string(),
            "".to_string(),
        );

        let name = &bill.name;

        write_bill_to_file(&bill);
        let bill_from_file = read_bill_from_file(name);

        assert_eq!("fa".to_string(), bill_from_file.bill_jurisdiction);
    }

    #[test]
    fn create_new_bill() {
        let bill = issue_new_bill(
            "".to_string(),
            "".to_string(),
            0,
            Identity {
                name: "".to_string(),
                date_of_birth: "".to_string(),
                city_of_birth: "".to_string(),
                country_of_birth: "".to_string(),
                email: "".to_string(),
                postal_address: "".to_string(),
                public_key_pem: "".to_string(),
                private_key_pem: "".to_string(),
            },
            "".to_string(),
            "".to_string(),
        );

        let rsa = generation_rsa_key();
        let private_key: Vec<u8> = rsa.public_key_to_pem().unwrap();
        let a = sha256(private_key.as_slice()).to_vec();
        let s = format!("{:?}", &a);
        let path = "test/".to_owned() + &s.replace(", ", "").replace("[", "").replace("]", "");
        fs::write(path.as_str(), "adsadsadsad".as_bytes()).expect("Unable to write bill in file.");
    }

    #[test]
    fn structure_to_bytes() {
        let ed25519_keys = Keypair::generate_ed25519();
        let peer_id = PeerId::from(ed25519_keys.public());
        let id = create_new_identity(
            "qwq".to_string(),
            "ewqe".to_string(),
            "qwewqe".to_string(),
            "qwewqe".to_string(),
            "qweeq".to_string(),
            "qwewqe".to_string(),
        );

        let bytes_ed25519_keys = unsafe { structure_as_u8_slice(&ed25519_keys) };
        let bytes_peer_id = unsafe { structure_as_u8_slice(&peer_id) };

        let bytes_ed25519_keys_sized = byte_array_to_size_array_keypair(bytes_ed25519_keys);
        let bytes_peer_id_sized = byte_array_to_size_array_peer_id(bytes_peer_id);

        if !Path::new("test").exists() {
            fs::create_dir("test").expect("Can't create folder.");
        }
        fs::write("test/keys", *bytes_ed25519_keys_sized).expect("Unable to write keys in file");
        fs::write("test/peer_id", *bytes_peer_id_sized).expect("Unable to write peer id in file");

        let data_key = fs::read("test/keys").expect("Unable to read file with keypair");
        let key_pair_bytes_sized = byte_array_to_size_array_keypair(data_key.as_slice());
        let key_pair2: Keypair = unsafe { mem::transmute_copy(key_pair_bytes_sized) };

        let data_peer_id = fs::read("test/peer_id").expect("Unable to read file with peer_id");
        let peer_id_bytes_sized = byte_array_to_size_array_peer_id(data_peer_id.as_slice());
        let peer_id2: PeerId = unsafe { mem::transmute_copy(peer_id_bytes_sized) };
    }

    #[test]
    fn encrypt_bill_with_rsa_keypair() {
        let bill = issue_new_bill(
            "".to_string(),
            "".to_string(),
            0,
            Identity {
                name: "".to_string(),
                date_of_birth: "".to_string(),
                city_of_birth: "".to_string(),
                country_of_birth: "".to_string(),
                email: "".to_string(),
                postal_address: "".to_string(),
                public_key_pem: "".to_string(),
                private_key_pem: "".to_string(),
            },
            "".to_string(),
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
            "".to_string(),
            "".to_string(),
            0,
            Identity {
                name: "".to_string(),
                date_of_birth: "".to_string(),
                city_of_birth: "".to_string(),
                country_of_birth: "".to_string(),
                email: "".to_string(),
                postal_address: "".to_string(),
                public_key_pem: "".to_string(),
                private_key_pem: "".to_string(),
            },
            "".to_string(),
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
        let data = issue_new_bill(
            "".to_string(),
            "".to_string(),
            0,
            Identity {
                name: "".to_string(),
                date_of_birth: "".to_string(),
                city_of_birth: "".to_string(),
                country_of_birth: "".to_string(),
                email: "".to_string(),
                postal_address: "".to_string(),
                public_key_pem: "".to_string(),
                private_key_pem: "".to_string(),
            },
            "".to_string(),
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
    fn bill_to_bytes_and_opposite_with_borsh() {
        let bill = issue_new_bill(
            "".to_string(),
            "".to_string(),
            0,
            Identity {
                name: "".to_string(),
                date_of_birth: "".to_string(),
                city_of_birth: "".to_string(),
                country_of_birth: "".to_string(),
                email: "".to_string(),
                postal_address: "".to_string(),
                public_key_pem: "".to_string(),
                private_key_pem: "".to_string(),
            },
            "".to_string(),
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
        let result = encode(&123_324_324);
        assert_eq!("one hundred twenty-three million three hundred twenty-four thousand three hundred twenty-four".to_string(), result);
    }
}
