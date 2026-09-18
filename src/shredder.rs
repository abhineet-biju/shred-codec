//! Encodes input bytes into checksummed data and Reed–Solomon coding shreds.
use reed_solomon_erasure::galois_8::ReedSolomon;

use crate::error::Error;

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub data_shreds: usize,
    pub coding_shreds: usize,
    pub shred_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_shreds: 8,
            coding_shreds: 8,
            shred_size: 128,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PayloadMetaData {
    pub original_len: usize,
    pub group_count: usize,
    pub config: Config,
}

#[derive(Debug, Clone)]
pub struct Shred {
    pub group_index: usize,
    pub index: usize,
    pub data: Vec<u8>,
    pub checksum: u32,
}

#[derive(Debug, Clone)]
pub struct ShredBatch {
    pub metadata: PayloadMetaData,
    pub shreds: Vec<Shred>,
}

pub fn shred(input: &[u8], config: &Config) -> Result<ShredBatch, Error> {
    if config.shred_size == 0 {
        return Err(Error::InvalidConfig("shred size cannot be 0"));
    }

    let codec = ReedSolomon::new(config.data_shreds, config.coding_shreds)?;
    let group_capacity = config
        .data_shreds
        .checked_mul(config.shred_size)
        .ok_or(Error::InvalidConfig("group capacity overflows usize"))?;

    let total_shreds = config.data_shreds + config.coding_shreds;
    let group_count = input.len().div_ceil(group_capacity);

    let mut shreds = Vec::new();

    for (group_index, group) in input.chunks(group_capacity).enumerate() {
        // Allocate equal sized buffers for both data shreds and coding shreds
        let mut shard_buffers = vec![vec![0u8; config.shred_size]; total_shreds];

        // Copy input to the data buffers
        // Coding buffer remains empty and is filled later by the encoder
        for (index, bytes) in group.chunks(config.shred_size).enumerate() {
            shard_buffers[index][..bytes.len()].copy_from_slice(bytes);
        }

        codec.encode(&mut shard_buffers)?;

        for (index, shard) in shard_buffers.into_iter().enumerate() {
            let checksum = crc32fast::hash(&shard);

            shreds.push(Shred {
                group_index,
                data: shard,
                index,
                checksum,
            });
        }
    }

    Ok(ShredBatch {
        metadata: PayloadMetaData {
            original_len: input.len(),
            group_count,
            config: *config,
        },
        shreds,
    })
}
