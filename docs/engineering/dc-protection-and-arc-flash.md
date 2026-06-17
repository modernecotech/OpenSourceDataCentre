# DC Protection and Arc-Flash

The 380-400 VDC architecture is a core OSDC concept and a commercial acceptance risk.

Before deployment, every project must produce:

- short-circuit and fault-current model;
- DC breaker and fuse selectivity study;
- arc-flash and arc-fault mitigation assessment;
- certified DC-rated switching and protective devices;
- isolation monitoring plan;
- emergency disconnect and controlled shutdown sequence;
- safe approach boundaries and labels;
- lockout/tagout procedure;
- commissioning test plan.

Design principles:

- Use certified components for the actual voltage, current, fault, and environmental conditions.
- Keep safety-critical tripping in approved protective devices and controllers.
- Use Rust/portal systems for planning, evidence, workflow, and audit only.
- Treat every maintenance procedure as a controlled MOP with rollback and safety hold points.
