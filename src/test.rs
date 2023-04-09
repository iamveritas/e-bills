#[cfg(test)]
mod test {
    use std::{fs, mem};
    use std::io::{BufReader, Cursor, Read};
    use std::path::{Path, PathBuf};
    use std::time::Duration;

    use borsh::{BorshDeserialize, BorshSerialize};
    use libp2p::{identity, PeerId};
    use libp2p::identity::Keypair;
    use libp2p::kad::{Kademlia, KademliaConfig};
    use libp2p::kad::store::MemoryStore;
    use openssl::{aes, rsa, sha};
    use openssl::hash::MessageDigest;
    use openssl::pkey::{PKey, Private, Public};
    use openssl::rsa::{Padding, Rsa};
    use openssl::sha::sha256;
    use openssl::sign::{Signer, Verifier};
    use openssl::symm::Cipher;

    use crate::{
        bill_from_byte_array, bill_to_byte_array, BitcreditBill,
        byte_array_to_size_array_keypair, byte_array_to_size_array_peer_id, create_new_identity, decrypt_bytes,
        encrypt_bytes, generation_rsa_key, hash_data_from_bill, Identity,
        issue_new_bill, pem_private_key_from_rsa, pem_public_key_from_rsa,
        private_key_from_pem_u8, public_key_from_pem_u8, read_bill_from_file, read_identity_from_file,
        structure_as_u8_slice,
    };
    use crate::blockchain::{Block, Chain, is_block_valid, signature};
    use crate::constants::BILLS_FOLDER_PATH;
    use crate::numbers_to_words::encode;

//DONT GO IN PIPELINE
    // #[test]
    // fn blockchain() {
    //     //Identity
    //     let drawer = read_identity_from_file();
    //
    //     // New bill
    //     let bill = issue_new_bill(
    //         "bill.bill_jurisdiction".to_string(),
    //         "bill.place_of_drawing".to_string(),
    //         10,
    //         drawer.clone(),
    //         "bill.language".to_string(),
    //         "bill.drawee_name".to_string(),
    //     );
    //
    //     // Read blockchain from file
    //     let mut blockchain_from_file = Chain::read_chain_from_file(&bill.name);
    //
    //     //Take last block
    //     let last_block = blockchain_from_file.get_latest_block();
    //
    //     // Data for second block
    //     let data2 = "Ivan Tymko".to_string();
    //
    //     // Create second block
    //     let private_key = private_key_from_pem_u8(&drawer.private_key_pem.as_bytes().to_vec());
    //     let signer_key = PKey::from_rsa(private_key).unwrap();
    //     let signature: String = signature(&bill, &signer_key);
    //     let block_two = Block::new(
    //         last_block.id + 1,
    //         last_block.hash.clone(),
    //         hex::encode(data2.as_bytes()),
    //         bill.name.clone(),
    //         signature,
    //         "".to_string(),
    //         "".to_string(),
    //     );
    //
    //     // Validate and write chain
    //     blockchain_from_file.try_add_block(block_two);
    //     if blockchain_from_file.is_chain_valid() {
    //         blockchain_from_file.write_chain_to_file(&bill.name);
    //     }
    //
    //     // Try take last version of bill
    //     let chain_two = Chain::read_chain_from_file(&bill.name);
    //     let bill2 = chain_two.get_last_version_bill();
    //
    //     //Tests
    //     assert_eq!(bill.holder_name, "Mykyta Tymko".to_string());
    //     assert_eq!(bill2.holder_name, "Ivan Tymko".to_string());
    // }

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
        let bill = BitcreditBill {
            name: "".to_string(),
            to_payee: false,
            bill_jurisdiction: "".to_string(),
            timestamp_at_drawing: 0,
            drawee_name: "".to_string(),
            drawer_name: "".to_string(),
            holder_name: "".to_string(),
            place_of_drawing: "".to_string(),
            currency_code: "".to_string(),
            amount_numbers: 0,
            amounts_letters: "".to_string(),
            maturity_date: "".to_string(),
            date_of_issue: "".to_string(),
            compounding_interest_rate: 0,
            type_of_interest_calculation: false,
            place_of_payment: "".to_string(),
            public_key_pem: "".to_string(),
            private_key_pem: "".to_string(),
            language: "".to_string(),
        };

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
        let bill = BitcreditBill {
            name: "".to_string(),
            to_payee: false,
            bill_jurisdiction: "".to_string(),
            timestamp_at_drawing: 0,
            drawee_name: "".to_string(),
            drawer_name: "".to_string(),
            holder_name: "".to_string(),
            place_of_drawing: "".to_string(),
            currency_code: "".to_string(),
            amount_numbers: 0,
            amounts_letters: "".to_string(),
            maturity_date: "".to_string(),
            date_of_issue: "".to_string(),
            compounding_interest_rate: 0,
            type_of_interest_calculation: false,
            place_of_payment: "".to_string(),
            public_key_pem: "".to_string(),
            private_key_pem: "".to_string(),
            language: "".to_string(),
        };

        let rsa_key = generation_rsa_key();
        let bill_bytes = bill_to_byte_array(&bill);

        let encrypted_bill = encrypt_bytes(&bill_bytes, &rsa_key);

        let decrypted_bill = decrypt_bytes(&encrypted_bill, &rsa_key);
        assert_eq!(bill_bytes.len(), decrypted_bill.len());

        let new_bill = bill_from_byte_array(&decrypted_bill);

        assert_eq!(bill.bill_jurisdiction, new_bill.bill_jurisdiction);
    }

    #[test]
    fn sign_and_verify_data_given_an_rsa_keypair() {
        let data = BitcreditBill {
            name: "".to_string(),
            to_payee: false,
            bill_jurisdiction: "".to_string(),
            timestamp_at_drawing: 0,
            drawee_name: "".to_string(),
            drawer_name: "".to_string(),
            holder_name: "".to_string(),
            place_of_drawing: "".to_string(),
            currency_code: "".to_string(),
            amount_numbers: 0,
            amounts_letters: "".to_string(),
            maturity_date: "".to_string(),
            date_of_issue: "".to_string(),
            compounding_interest_rate: 0,
            type_of_interest_calculation: false,
            place_of_payment: "".to_string(),
            public_key_pem: "".to_string(),
            private_key_pem: "".to_string(),
            language: "".to_string(),
        };

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
    fn numbers_to_letters() {
        let result = encode(&123_324_324);
        assert_eq!("one hundred twenty-three million three hundred twenty-four thousand three hundred twenty-four".to_string(), result);
    }
}
