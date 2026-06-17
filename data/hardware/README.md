# Hardware Data

This directory stores machine-readable hardware planning and provisioning data.

- `compute-baseline-2026.csv` lists candidate compute profiles and baseline hardware assumptions.
- `provisioning-pipeline.csv` lists the rack-to-running stages for hardware requests.
- `provisioning-profiles.csv` lists user-facing hardware provisioning profiles and target pools.
- `provisioning-requests.csv` provides sample hardware request queue records for the portal prototype.

Keep these files as simple rectangular CSVs with one header row. Provisioning profiles should reference clear target pools such as OpenStack compute, Kubernetes GPU, Ceph storage, Edge Shield, data platform, or facility OT.
