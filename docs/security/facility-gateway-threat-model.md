# Facility Gateway Threat Model

Facility gateways bridge OT telemetry and IT systems, so they are sensitive.

Threats:

- unauthorized command path to controllers;
- credential theft;
- telemetry spoofing;
- denial of monitoring;
- unsafe remote access;
- lateral movement from IT to OT;
- supply-chain compromise.

Controls:

- read-only by default;
- allowlisted protocols;
- strong identity;
- signed updates;
- local logging;
- network segmentation;
- backup and restore;
- tested fail-safe behavior.
