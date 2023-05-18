use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ChainStats {
    funded_txo_count: u64,
    pub funded_txo_sum: u64,
    spent_txo_count: u64,
    pub spent_txo_sum: u64,
    tx_count: u64,
}

#[derive(Deserialize, Debug)]
pub struct MempoolStats {
    funded_txo_count: u64,
    funded_txo_sum: u64,
    spent_txo_count: u64,
    spent_txo_sum: u64,
    tx_count: u64,
}

#[derive(Deserialize, Debug)]
pub struct AddressInfo {
    address: String,
    pub chain_stats: ChainStats,
    mempool_stats: MempoolStats,
}

impl AddressInfo {
    pub async fn get_testnet_address_info(address: String) -> Self {
        let request_url = format!(
            "https://blockstream.info/testnet/api/address/{address}",
            address = address
        );
        let address: AddressInfo = reqwest::get(&request_url)
            .await
            .expect("Failed to send request")
            .json()
            .await
            .expect("Failed to read response");

        address
    }

    async fn get_mainnet_address_info(address: String) -> Self {
        let request_url = format!(
            "https://blockstream.info/api/address/{address}",
            address = address
        );
        let address: AddressInfo = reqwest::get(&request_url)
            .await
            .expect("Failed to send request")
            .json()
            .await
            .expect("Failed to read response");

        address
    }
}
