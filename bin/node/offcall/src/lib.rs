use sc_transaction_pool::txpool::{self, ExtrinsicFor};
use sp_transaction_pool::{TransactionPool, InPoolTransaction};
// use futures::prelude::*;
use futures::{future, Future};
use futures::StreamExt;
use sp_runtime::{
    codec::{Decode, Encode},
    generic::{BlockId, Era},
    traits::{Block},
};
use sc_client::{blockchain::HeaderBackend, BlockchainEvents};
use sp_storage::{StorageChangeSet, StorageData, StorageKey};

use std::sync::Arc;
use sp_core::storage::well_known_keys;

fn prefix_key(module: &[u8], name: &[u8]) -> Vec<u8> {
    let mut key = [0u8;32];
    key[0..16].copy_from_slice(&sp_core::hashing::twox_128(module));
    key[16..].copy_from_slice(&sp_core::hashing::twox_128(name));
    key.to_vec()
}

pub fn start_offcall<A, B, C>(client: Arc<C>, pool: Arc<A>) -> impl Future<Output=()>
    where
        A:TransactionPool<Block = B>,
        B: Block,
        C: BlockchainEvents<B>
{
    let events_key = StorageKey(prefix_key(b"System", b"Events"));
    let stream = client.storage_changes_notification_stream(Some(&[events_key]), None)
        .unwrap()
        .for_each(|data| {
            println!("data: {:?}", data);
            future::ready(())
        });

    stream
}

#[cfg(test)]
mod tests {

}