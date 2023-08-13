// subxt metadata -f bytes > ./artifacts/metadata.scale
// #[subxt::subxt(runtime_metadata_url = "ws://localhost:9944")]
#[subxt::subxt(runtime_metadata_path = "./artifacts/metadata.scale")]
pub mod polkadot {}

pub mod id_types;
