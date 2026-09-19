use shred_codec::{Config, reconstruct, shred, validate_shred, validate_shred_batch};

#[test]
fn validates_and_reconstructs_a_multi_group_payload_after_shred_loss() {
    let config = Config {
        data_shreds: 4,
        coding_shreds: 2,
        shred_size: 8,
    };
    let original: Vec<u8> = (0..75).map(|value| ((value * 37) % 256) as u8).collect();

    let batch = shred(&original, &config).expect("shredding should succeed");

    let group_capacity = config.data_shreds * config.shred_size;
    let shreds_per_group = config.data_shreds + config.coding_shreds;
    let expected_group_count = original.len().div_ceil(group_capacity);

    assert_eq!(batch.metadata.original_len, original.len());
    assert_eq!(batch.metadata.group_count, expected_group_count);
    assert_eq!(batch.shreds.len(), expected_group_count * shreds_per_group);
    assert!(validate_shred_batch(&batch));

    // Remove two data shreds from every group. Because each group has two
    // coding shreds, the remaining four shreds are sufficient for recovery.
    let mut received = batch
        .shreds
        .iter()
        .filter(|shred| shred.index != 0 && shred.index != 2)
        .cloned()
        .collect::<Vec<_>>();

    let removed_per_group = 2;
    assert_eq!(
        batch.shreds.len() - received.len(),
        expected_group_count * removed_per_group
    );
    assert!(received.iter().all(validate_shred));

    // A receiver must use the shred identifiers rather than arrival order.
    received.reverse();

    let reconstructed = reconstruct(&batch.metadata, &received)
        .expect("the coding shreds should recover the missing data shreds");

    assert_eq!(reconstructed.len(), original.len());
    assert_eq!(reconstructed, original);
}
