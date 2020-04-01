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
use node_primitives::Hash;

use std::sync::Arc;
use sp_core::storage::well_known_keys;
use frame_system::{self as system, EventRecord};
use node_runtime::Event;

trait PrefixKey {
    fn as_prefix_key(&self) -> Vec<u8>;
}

impl PrefixKey for [u8] {
    fn as_prefix_key(&self) -> Vec<u8> {
        let mut key = [0u8;32];
        let mut items = self.split(|spa| *spa == b' ');
        if let Some(module) = items.next() {
            key[0..16].copy_from_slice(&sp_core::hashing::twox_128(module));
        }
        if let Some(name) = items.next() {
            key[16..].copy_from_slice(&sp_core::hashing::twox_128(name));
        }
        key.to_vec()
    }
}

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
    // let events_key = StorageKey(prefix_key(b"System", b"Events"));
    let events_key = StorageKey(b"System Events".as_prefix_key());
    let stream = client.storage_changes_notification_stream(Some(&[events_key]), None)
        .unwrap()
        .for_each(|data| {
            // println!("data: {:?}", data);
            let (block, change_set) = data;
            let changes = StorageChangeSet {
                block: block,
                changes: change_set
                .iter()
                .filter_map(|(o_sk, k, v)| {
                    if o_sk.is_none() {
                        Some((k.clone(), v.cloned()))
                    } else {
                        None
                    }
                })
                .collect()};

            let records: Vec<Vec<EventRecord<Event, Hash>>> = changes.changes
                .iter()
                .filter_map(|(_, mbdata)| {
                    if let Some(StorageData(data)) = mbdata {
                        Decode::decode(&mut &data[..]).ok()
                    } else {
                        None
                    }
                })
                .collect();

            println!("records: {:?}", records);
            future::ready(())
        });

    stream
}

#[cfg(test)]
mod tests {

}