# RDMA, RoCE, and InfiniBand Design

Large AI training can require RDMA-capable fabrics.

Decision points:

- Ethernet RoCE versus InfiniBand;
- congestion control;
- lossless or near-lossless configuration;
- GPU direct storage path;
- fabric telemetry;
- tenant isolation model;
- operational skill requirement;
- vendor lock-in risk;
- spares and optics availability.

Do not treat RDMA as a default for small pilots. It should be justified by workload and operator skill.
