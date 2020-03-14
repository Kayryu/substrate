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

pub fn start_offcall<A, B, C>(client: Arc<C>, pool: Arc<A>) -> impl Future<Output=()>
    where
        A:TransactionPool<Block = B>,
        B: Block,
        C: BlockchainEvents<B>
{
    println!("########### start offcall ##########");
    let events_key = StorageKey(sp_core::twox_128(b"System Events").to_vec());
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