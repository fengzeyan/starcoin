// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::storage::{ColumnFamily, InnerStorage, KVStore};
use crate::CHAIN_INFO_PREFIX_NAME;
use anyhow::Result;
use crypto::HashValue;
use starcoin_types::startup_info::StartupInfo;
use std::convert::TryInto;

#[derive(Clone)]
pub struct ChainInfoColumnFamily;

impl ColumnFamily for ChainInfoColumnFamily {
    type Key = String;
    type Value = Vec<u8>;

    fn name() -> &'static str {
        CHAIN_INFO_PREFIX_NAME
    }
}

pub type ChainInfoStorage = InnerStorage<ChainInfoColumnFamily>;

impl ChainInfoStorage {
    const STARTUP_INFO_KEY: &'static str = "startup_info";
    const GENESIS_KEY: &'static str = "genesis";

    pub fn get_startup_info(&self) -> Result<Option<StartupInfo>> {
        self.get(Self::STARTUP_INFO_KEY.as_bytes())
            .and_then(|bytes| match bytes {
                Some(bytes) => Ok(Some(bytes.try_into()?)),
                None => Ok(None),
            })
    }

    pub fn save_startup_info(&self, startup_info: StartupInfo) -> Result<()> {
        self.put(
            Self::STARTUP_INFO_KEY.as_bytes().to_vec(),
            startup_info.try_into()?,
        )
    }

    pub fn get_genesis(&self) -> Result<Option<HashValue>> {
        self.get(Self::GENESIS_KEY.as_bytes())
            .and_then(|bytes| match bytes {
                Some(bytes) => Ok(Some(HashValue::from_slice(bytes.as_slice())?)),
                None => Ok(None),
            })
    }

    pub fn save_genesis(&self, genesis_block_hash: HashValue) -> Result<()> {
        self.put(
            Self::GENESIS_KEY.as_bytes().to_vec(),
            genesis_block_hash.to_vec(),
        )
    }
}
