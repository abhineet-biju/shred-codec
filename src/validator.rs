use crate::shredder::{Shred, ShredBatch};

pub fn validate_shred(shred: &Shred) -> bool {
    crc32fast::hash(&shred.data) == shred.checksum
}

pub fn validate_shred_batch(shredbatch: &ShredBatch) -> bool {
    for shred in &shredbatch.shreds {
        if crc32fast::hash(&shred.data) != shred.checksum {
            return false;
        }
    }
    true
}
