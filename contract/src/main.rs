/*
Inspiration for this code:
- Moonbeam tutorial: https://docs.moonbeam.network/builders/build/eth-api/libraries/ethersrs/
- Ethers.rs examples: https://docs.rs/crate/ethers/1.0.2/source/examples/
- Contract was deployed using https://remix.ethereum.org/
*/

use ethers::providers::{Http, Provider};
use ethers::{prelude::*, types::U256};
use hex::ToHex;
use rand::Rng;
use sha256::digest;
use std::convert::TryInto;
use std::sync::Arc;

// Add client type
type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

const CONTRACT_ADDRESS: &str = "0xbfeC2320Bf24A289d393932fB30998Ed5fa84C46";
const ALCHEMY_MUMBAI_API_KEY: &str =
    "https://polygon-mumbai.g.alchemy.com/v2/VDEtXZglGFw5AoR48KaAj-ngFWYUehMY";

// ---> DO NOT USE THIS ACCOUNT ON MAINNET OR IN PUBLIC REPO !!!
const OWNER_PRIVATE_KEY: &str = "6aea615c3d873b52514bed23b33011ffea8d1e99a242fa3d459b24dc21c93f3d"; 
const OWNER_ADDRESS: &str = "0x6c29233170269A10283c15ee2dFbB2d70Ba4E5B7"; 

// Generates a type-safe interface for the Wordle smart contract
// Getters are set automatically for public variables, like commitment is (meaning you won't find commitment() getter in Wordle.sol)
abigen!(
    WordleContract,
    r"[
    function setOwner(address)
    function setCommitment(bytes32 commitment)
    function getOwner() external view returns (address)
    function commitment() external view returns (bytes32) 
    ]"
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(ALCHEMY_MUMBAI_API_KEY)?;

    // Use a private key to create a wallet
    // Do not include the private key in plain text in any production code
    // This is just for demonstration purposes
    // Do not include '0x' at the start of the private key
    let wallet: LocalWallet = OWNER_PRIVATE_KEY
        .parse::<LocalWallet>()?
        .with_chain_id(Chain::PolygonMumbai);

    // Wrap the provider and wallet together to create a signer client
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    // Add from and to address
    let address_owner = OWNER_ADDRESS.parse::<Address>()?;
    let address_contract = CONTRACT_ADDRESS.parse::<Address>()?;

    // Call some web3 functions as a demonstration
    print_balance(&provider, &address_owner).await?;
    read_owner(&client, &address_contract).await?;
    read_commitment(&client, &address_contract).await?;
    set_commitment(&client, &address_contract).await?;
    Ok(())
}

async fn print_balance(
    provider: &Provider<Http>,
    address: &Address,
) -> Result<(), Box<dyn std::error::Error>> {
    let balance = provider.get_balance(address.clone(), None).await?;
    println!("\n{} has {}", address, balance);
    Ok(())
}

async fn read_owner(
    client: &Client,
    contract_addr: &H160,
) -> Result<H160, Box<dyn std::error::Error>> {
    let contract = WordleContract::new(contract_addr.clone(), Arc::new(client.clone()));
    let owner = contract.get_owner().call().await?;

    println!("\nContract owner is {}", owner);
    Ok(owner)
}

async fn read_commitment(
    client: &Client,
    contract_addr: &H160,
) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    let contract = WordleContract::new(contract_addr.clone(), Arc::new(client.clone()));
    let commitment = contract.commitment().call().await?;
    let commitment_hex = commitment.encode_hex::<String>();

    println!("\nCommitment bytes is {:?}", commitment);
    println!("Commitment hex is {}", commitment_hex);

    Ok(commitment)
}

async fn set_commitment(
    client: &Client,
    contract_addr: &H160,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nSetting new commitment...");

    // 1. Pick word and salt
    let mut rng = rand::thread_rng();
    let word = "black";
    let salt_bytes: [u8; 32] = rng.gen(); // random 256 bits
    let salt_hex = hex::encode(salt_bytes);

    // 2. Compute commitment
    let commitment_hex = digest(word.to_string() + &salt_hex);
    let commitment_vec = hex::decode(&commitment_hex).unwrap();
    let commitment_bytes: [u8; 32] = commitment_vec.try_into().unwrap(); 

    println!("word: {}", word);
    println!("salt: {}", salt_hex);
    println!("commitment_hex: {}", commitment_hex);
    println!("commitment_bytes: {:?}", commitment_bytes);

    // 3. Create contract instance
    let contract = WordleContract::new(contract_addr.clone(), Arc::new(client.clone()));

    // 4. Send transaction that updates commitment
    let tx = contract
        .set_commitment(commitment_bytes)
        .gas(U256::from(50000)) // Gas
        .gas_price(U256::from(10_000_000_000u128)) // 10 Gwei - set experimentally. 1 Gwei is too little
        .send()
        .await?
        .await?;

    println!("\nTransaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}
