use reed_solomon_erasure::galois_8::ReedSolomon;

use crate::{
    error::Error,
    shredder::{PayloadMetaData, Shred},
    validator::validate_shred,
};

pub fn reconstruct(metadata: &PayloadMetaData, shreds: &[Shred]) -> Result<Vec<u8>, Error> {
    let config = &metadata.config;

    if config.shred_size == 0 {
        return Err(Error::InvalidConfig("shred size cannot be 0"));
    }

    let codec = ReedSolomon::new(config.data_shreds, config.coding_shreds)?;

    let shreds_per_group = config.data_shreds + config.coding_shreds;

    let mut groups = vec![vec![None; shreds_per_group]; metadata.group_count];

    for shred in shreds {
        if shred.group_index >= metadata.group_count {
            return Err(Error::InvalidGroupIndex);
        }

        if shred.index >= shreds_per_group {
            return Err(Error::InvalidShredIndex);
        }

        if shred.data.len() != config.shred_size {
            return Err(Error::InvalidShredSize);
        }

        if !validate_shred(shred) {
            continue;
        }

        let slot = &mut groups[shred.group_index][shred.index];

        // Reject if conflicting shred exists in same slot
        if let Some(existing) = slot.as_ref() {
            if existing == &shred.data {
                // Identical duplicate.
                continue;
            }

            return Err(Error::ConflictingShred {
                group_index: shred.group_index,
                index: shred.index,
            });
        }

        *slot = Some(shred.data.clone());
    }

    let mut reconstructed = Vec::new();

    for (group_index, group) in groups.iter_mut().enumerate() {
        codec.reconstruct_data(group)?;

        for (index, slot) in group.iter().take(config.data_shreds).enumerate() {
            let data = slot
                .as_ref()
                .ok_or(Error::MissingDataShred { group_index, index })?;

            reconstructed.extend_from_slice(data);
        }
    }

    reconstructed.truncate(metadata.original_len);

    Ok(reconstructed)
}
