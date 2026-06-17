# BMS and EPMS Network Segmentation

BMS and EPMS networks must be isolated from tenant, office, and public networks.

Controls:

- separate VLANs or physical networks;
- firewall policy;
- jump host for administration;
- no direct internet access from controllers;
- allowlisted telemetry flows;
- read-only default integration to portal;
- MFA for privileged access;
- log collection and time synchronization.

Segmentation must be tested during commissioning and after major changes.
