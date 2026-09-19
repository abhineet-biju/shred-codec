<h1 align="center">shred-codec</h1>

`shred-codec` is a rust library for shredding and reconstructing data using Reed-Solomon erasure coding. It takes raw bytes, splits them into data and coding shreds, validates their checksums, and recovers the original data when enough valid shreds remain in each group.

<p align="center">
  <img src="docs/images/shred-codec-overview.svg" alt="Shred-codec overview showing shredding, checksum validation, and data reconstruction" width="600" />
</p>
