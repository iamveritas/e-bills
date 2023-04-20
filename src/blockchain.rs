use borsh::BorshSerialize;
use chrono::prelude::*;
use log::{info, warn};
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::sha::Sha256;
use openssl::sign::{Signer, Verifier};
use serde::{Deserialize, Serialize};

use crate::{
    bill_from_byte_array, bill_to_byte_array, BitcreditBill, private_key_from_pem_u8,
    public_key_from_pem_u8,
};
use crate::constants::BILLS_FOLDER_PATH;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chain {
    pub blocks: Vec<Block>,
}

impl Chain {
    pub fn new(first_block: Block) -> Self {
        let mut blocks = Vec::new();
        blocks.push(first_block);

        Self { blocks }
    }

    pub fn read_chain_from_file(bill_name: &String) -> Self {
        let input_path = BILLS_FOLDER_PATH.to_string() + "/" + bill_name.as_str() + ".json";
        let blockchain_from_file = std::fs::read(input_path.clone()).expect("file not found");
        serde_json::from_slice(blockchain_from_file.as_slice()).unwrap()
    }

    pub fn write_chain_to_file(&self, bill_name: &String) {
        let output_path = BILLS_FOLDER_PATH.to_string() + "/" + bill_name.as_str() + ".json";
        std::fs::write(
            output_path.clone(),
            serde_json::to_string_pretty(&self).unwrap(),
        )
            .unwrap();
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 0..self.blocks.len() {
            if i == 0 {
                continue;
            }
            let first: &Block = &self.blocks[i - 1];
            let second: &Block = &self.blocks[i];
            if !is_block_valid(second, first) {
                return false;
            }
        }
        true
    }

    pub fn try_add_block(&mut self, block: Block) {
        let latest_block = self.blocks.last().expect("there is at least one block");
        if is_block_valid(&block, latest_block) {
            self.blocks.push(block);
        } else {
            error!("could not add block - invalid");
        }
    }

    pub fn get_latest_block(&self) -> &Block {
        self.blocks.last().expect("there is at least one block")
    }

    pub fn get_first_block(&self) -> &Block {
        self.blocks.first().expect("there is at least one block")
    }

    pub fn get_last_version_block_with_operation_code(
        &self,
        operation_code: OperationCode,
    ) -> &Block {
        let mut last_version_block: &Block = &self.get_first_block();
        for block in &self.blocks {
            if block.operation_code == operation_code {
                last_version_block = block;
            }
        }
        last_version_block
    }

    pub fn exist_block_with_operation_code(&self, operation_code: OperationCode) -> bool {
        let mut exist_block_with_operation_code = false;
        for block in &self.blocks {
            if block.operation_code == operation_code {
                exist_block_with_operation_code = true;
            }
        }
        exist_block_with_operation_code
    }

    pub fn get_last_version_bill_with_operation_code(&self, operation_code: OperationCode) -> BitcreditBill {
        let first_block_data = &self.get_first_block().data;
        let bill_first_version_in_bytes = hex::decode(first_block_data).unwrap();
        let bill_first_version: BitcreditBill = bill_from_byte_array(&bill_first_version_in_bytes);

        let mut holder_bill_last_version: String = bill_first_version.holder_name.clone();

        if self.blocks.len() > 1 && self.exist_block_with_operation_code(operation_code.clone()) {
            let last_version_block =
                self.get_last_version_block_with_operation_code(operation_code);
            let last_version_block_data = &last_version_block.data;
            let holder_bill_last_version_u8 = hex::decode(last_version_block_data).unwrap();
            holder_bill_last_version = String::from_utf8(holder_bill_last_version_u8).unwrap();
        }

        let bill = BitcreditBill {
            name: bill_first_version.name,
            to_payee: bill_first_version.to_payee,
            bill_jurisdiction: bill_first_version.bill_jurisdiction,
            timestamp_at_drawing: bill_first_version.timestamp_at_drawing,
            drawee_name: bill_first_version.drawee_name,
            drawer_name: bill_first_version.drawer_name,
            holder_name: holder_bill_last_version.clone(),
            place_of_drawing: bill_first_version.place_of_drawing,
            currency_code: bill_first_version.currency_code,
            amount_numbers: bill_first_version.amount_numbers,
            amounts_letters: bill_first_version.amounts_letters,
            maturity_date: bill_first_version.maturity_date,
            date_of_issue: bill_first_version.date_of_issue,
            compounding_interest_rate: bill_first_version.compounding_interest_rate,
            type_of_interest_calculation: bill_first_version.type_of_interest_calculation,
            place_of_payment: bill_first_version.place_of_payment,
            public_key_pem: bill_first_version.public_key_pem,
            private_key_pem: bill_first_version.private_key_pem,
            language: bill_first_version.language,
        };

        bill
    }

    pub fn get_last_version_bill(&self) -> BitcreditBill {
        let first_block_data = &self.get_first_block().data;
        let bill_first_version_in_bytes = hex::decode(first_block_data).unwrap();
        let bill_first_version: BitcreditBill = bill_from_byte_array(&bill_first_version_in_bytes);

        let mut holder_bill_last_version: String = bill_first_version.holder_name.clone();

        if self.blocks.len() > 1 {
            let last_block_data = &self.get_latest_block().data;
            let holder_bill_last_version_in_bytes = hex::decode(last_block_data).unwrap();
            holder_bill_last_version =
                String::from_utf8(holder_bill_last_version_in_bytes).unwrap();
        }

        let bill = BitcreditBill {
            name: bill_first_version.name,
            to_payee: bill_first_version.to_payee,
            bill_jurisdiction: bill_first_version.bill_jurisdiction,
            timestamp_at_drawing: bill_first_version.timestamp_at_drawing,
            drawee_name: bill_first_version.drawee_name,
            drawer_name: bill_first_version.drawer_name,
            holder_name: holder_bill_last_version.clone(),
            place_of_drawing: bill_first_version.place_of_drawing,
            currency_code: bill_first_version.currency_code,
            amount_numbers: bill_first_version.amount_numbers,
            amounts_letters: bill_first_version.amounts_letters,
            maturity_date: bill_first_version.maturity_date,
            date_of_issue: bill_first_version.date_of_issue,
            compounding_interest_rate: bill_first_version.compounding_interest_rate,
            type_of_interest_calculation: bill_first_version.type_of_interest_calculation,
            place_of_payment: bill_first_version.place_of_payment,
            public_key_pem: bill_first_version.public_key_pem,
            private_key_pem: bill_first_version.private_key_pem,
            language: bill_first_version.language,
        };

        bill
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum OperationCode {
    Issue,
    Accept,
    Decline,
    Endorse,
    RequestToAccept,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Block {
    pub id: u64,
    pub bill_name: String,
    pub hash: String,
    pub timestamp: i64,
    pub data: String,
    pub previous_hash: String,
    pub signature: String,
    pub public_key: String,
    pub operation_code: OperationCode,
}

impl Block {
    pub fn new(
        id: u64,
        previous_hash: String,
        data: String,
        bill_name: String,
        public_key: String,
        operation_code: OperationCode,
        private_key: String,
    ) -> Self {
        let now = Utc::now();
        let timestamp = now.timestamp();
        let hash: String = mine_block(
            &id,
            &bill_name,
            &previous_hash,
            &data,
            &timestamp,
            // &node_id,
            &public_key,
            &operation_code,
        );
        let signature = signature(hash.clone(), private_key.clone());

        Self {
            id,
            bill_name,
            hash,
            timestamp: now.timestamp(),
            previous_hash,
            signature,
            // node_id,
            data,
            public_key,
            operation_code,
        }
    }

    pub fn verifier(&self) -> bool {
        let public_key_bytes = self.public_key.as_bytes();
        let public_key_rsa = public_key_from_pem_u8(&public_key_bytes.to_vec());
        let verifier_key = PKey::from_rsa(public_key_rsa).unwrap();

        let mut verifier = Verifier::new(MessageDigest::sha256(), verifier_key.as_ref()).unwrap();

        let data_to_check = self.hash.as_bytes();
        verifier.update(data_to_check).unwrap();

        let signature_bytes = hex::decode(&self.signature).unwrap();
        verifier.verify(signature_bytes.as_slice()).unwrap()
    }
}

fn mine_block(
    id: &u64,
    bill_name: &str,
    previous_hash: &str,
    data: &str,
    timestamp: &i64,
    public_key: &str,
    operation_code: &OperationCode,
) -> String {
    let hash = calculate_hash(
        id,
        bill_name,
        previous_hash,
        data,
        timestamp,
        public_key,
        operation_code,
    );
    let binary_hash = hex::encode(&hash);
    info!(
        "mined! hash: {}, binary hash: {}",
        hex::encode(&hash),
        binary_hash
    );
    hex::encode(hash)
}

fn calculate_hash(
    id: &u64,
    bill_name: &str,
    previous_hash: &str,
    data: &str,
    timestamp: &i64,
    public_key: &str,
    operation_code: &OperationCode,
) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "bill_name": bill_name,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "public_key": public_key,
        "operation_code": operation_code,
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finish().try_to_vec().unwrap()
}

pub fn signature(hash: String, private_key_pem: String) -> String {
    let private_key_bytes = private_key_pem.as_bytes();
    let private_key_rsa = private_key_from_pem_u8(&private_key_bytes.to_vec());
    let signer_key = PKey::from_rsa(private_key_rsa).unwrap();

    let mut signer: Signer = Signer::new(MessageDigest::sha256(), signer_key.as_ref()).unwrap();

    let data_to_sign = hash.as_bytes();
    signer.update(data_to_sign).unwrap();

    let signature: Vec<u8> = signer.sign_to_vec().unwrap();
    let signature_readable = hex::encode(signature.as_slice());

    signature_readable
}

pub fn hash_data_from_bill(bill: &BitcreditBill) -> String {
    let bill_bytes: Vec<u8> = bill_to_byte_array(bill);
    let data_from_bill_hash_readable = hex::encode(bill_bytes);
    data_from_bill_hash_readable
}

pub fn start_blockchain_for_new_bill(
    bill: &BitcreditBill,
    operation_code: OperationCode,
    public_key: String,
    private_key: String,
) {
    let genesis_hash: String = hash_data_from_bill(&bill);

    let bill_data: String = hash_data_from_bill(&bill);

    let first_block = Block::new(
        1,
        genesis_hash,
        bill_data,
        bill.name.clone(),
        public_key,
        operation_code,
        private_key,
    );

    let chain = Chain::new(first_block);

    //Write chain to file
    let output_path = BILLS_FOLDER_PATH.to_string() + "/" + bill.name.clone().as_str() + ".json";
    std::fs::write(
        output_path.clone(),
        serde_json::to_string_pretty(&chain).unwrap(),
    )
        .unwrap();
}

pub fn is_block_valid(block: &Block, previous_block: &Block) -> bool {
    if block.previous_hash != previous_block.hash {
        warn!("block with id: {} has wrong previous hash", block.id);
        return false;
    } else if block.id != &previous_block.id + 1 {
        warn!(
            "block with id: {} is not the next block after the latest: {}",
            block.id, previous_block.id
        );
        return false;
    } else if hex::encode(calculate_hash(
        &block.id,
        &block.bill_name,
        &block.previous_hash,
        &block.data,
        &block.timestamp,
        &block.public_key,
        &block.operation_code,
    )) != block.hash
    {
        warn!("block with id: {} has invalid hash", block.id);
        return false;
    } else if !block.verifier() {
        warn!("block with id: {} has invalid signature", block.id);
        return false;
    }
    true
}
