use graph::{
    components::store::ChainStore as ChainStoreTrait,
    prelude::{
        anyhow::{self, Context},
        hex, serde_json,
        web3::types::H256,
    },
};
use graph_chain_ethereum::{EthereumAdapter, EthereumAdapterTrait};
use graph_store_postgres::ChainStore;
use std::io::{self, Write};
use std::sync::Arc;

pub async fn by_hash(
    chain_store: Arc<ChainStore>,
    ethereum_adapter: &EthereumAdapter,
    hash: &str,
) -> anyhow::Result<()> {
    // Create a BlockHash value to parse the input as a propper block hash
    let block_hash = {
        let hash = hash.trim_start_matches("0x");
        let hash = hex::decode(hash)
            .with_context(|| format!("Cannot parse H256 value from string `{}`", hash))?;
        H256::from_slice(&hash)
    };

    // Try to find a matching block from the store
    let blocks = chain_store.blocks(&[block_hash])?;
    match &blocks[..] {
        [_single_block] => { /* we want this case */ }
        [] => anyhow::bail!("Found no block with hash equal to {}", hash),
        _ => anyhow::bail!("Found multiple blocks with hash equal to {}", hash),
    };

    // Compare and report
    // let comparison_results =
    // compare_blocks(&blocks, ethereum_adapter)context("Failed to compare blocks")?;
    todo!("report comparison results")
}

pub async fn by_number(
    chain_store: Arc<ChainStore>,
    ethereum_adapter: &EthereumAdapter,
    number: i32,
) -> anyhow::Result<()> {
    let block_hashes = chain_store.block_hashes_by_block_number(number)?;

    // Try to resolve block number into a single block hash.
    let hash = match block_hashes[..] {
        [] => anyhow::bail!("Found no block with number {}", number),
        [hash] => hash,
        _ => anyhow::bail!(
            "Found multiple blocks for the same number. Please specify a block hash instead."
        ),
    };

    // Try to find a matching block from the store
    let blocks = chain_store.blocks(&[hash])?;
    match &blocks[..] {
        [_single_block] => { /* we want this case */ }
        [] => anyhow::bail!("Found no block with hash equal to {}", hash),
        _ => anyhow::bail!("Found multiple blocks with hash equal to {}", hash),
    };

    // Compare and report
    // let comparison_results =
    //     compare_blocks(&blocks, ethereum_adapter).with_context(|| "Failed to compare blocks")?;
    todo!("report comparison results")
}

pub async fn by_range(
    chain_store: Arc<ChainStore>,
    ethereum_adapter: &EthereumAdapter,
    range: &str,
) -> anyhow::Result<()> {
    todo!("resolve a range of block numbers into a collection of blocks");
    todo!("call `compare_blocks` function");
    todo!("report")
}

pub fn truncate(chain_store: Arc<ChainStore>, skip_confirmation: bool) -> anyhow::Result<()> {
    if !skip_confirmation && !prompt_for_confirmation()? {
        println!("Aborting.");
        return Ok(());
    }

    chain_store
        .truncate_block_cache()
        .with_context(|| format!("Failed to truncate block cache for {}", chain_store.chain))
}

fn compare_blocks(
    cached_block: &[serde_json::Value],
    block_hashes: &[H256],
    ethereum_adapter: &EthereumAdapter,
) -> anyhow::Result<()> {
    // let eth = ethereum_adapter.web3.eth();

    // eth.block_by_hash(&ethereum_adapter.logger, )
    todo!("call jrpc and collect fresh blocks for the given input set");

    todo!("diff the pairs");
}

fn prompt_for_confirmation() -> anyhow::Result<bool> {
    print!("This will delete all cached blocks.\nProceed? [y/N] ");
    io::stdout().flush()?;

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    answer.make_ascii_lowercase();

    match answer.trim() {
        "y" | "yes" => Ok(true),
        _ => Ok(false),
    }
}
