pub use crate::polkadot::{
    self,
    runtime_types::{
        bounded_collections::bounded_vec::BoundedVec,
        pallet_verification_protocol::{pallet::Error as PalletError, types::IdType},
    },
};

use subxt::{
    // utils::{AccountId32, MultiAddress},
    OnlineClient,
    PolkadotConfig,
};
use subxt_signer::sr25519::dev::{self};

impl IdType {
    fn build(name: Vec<u8>, issuer: Vec<u8>, country: Vec<u8>) -> Result<IdType, PalletError> {
        Ok(Self {
            name: BoundedVec(name),
            issuer: BoundedVec(issuer),
            country: BoundedVec(country),
        })
    }
    pub fn id_indian_passport() -> IdType {
        let id_type = IdType::build(
            b"PASSPORT".to_vec(),
            b"GOVTOFINDIA".to_vec(),
            b"INDIA".to_vec(),
        )
        .expect("error in creatign id type");

        id_type
    }
}

pub async fn whitelist_id_type(id_type: IdType) -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    println!("Connection with substrate node established.");

    // let alice: MultiAddress<AccountId32, ()> = dev::alice().public_key().into();

    let alice_pair_signer = dev::alice();

    let tx_whitelist = polkadot::tx()
        .verification_protocol()
        .whitelist_id_type(id_type);

    let _tx_events = api
        .tx()
        .sign_and_submit_then_watch_default(&tx_whitelist, &alice_pair_signer)
        .await
        .map(|e| {
            println!("ID whitelist submitted, waiting for transaction to be finalized...");
            e
        })?
        .wait_for_finalized_success()
        .await?;
    println!("ID Type whitelisted.");

    // api.tx().sign_and_submit_default(&tx_create_did, &signer_alice).await.unwrap();
    api.tx()
        .sign_and_submit_default(&tx_whitelist, &alice_pair_signer)
        .await
        .unwrap();
    Ok(())
}
