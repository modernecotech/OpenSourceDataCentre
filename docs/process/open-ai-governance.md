# Open AI Governance

The project should support AI models that can be used and shared without making operators dependent on a single vendor. That requires both technical queueing and license governance.

## Model Classification

Use these labels in the registry:

- `fully_open_source_ai`: meets the OSI Open Source AI Definition target, including the preferred form for modification.
- `open_weight`: weights are downloadable and reusable, but training data or full training process is not available.
- `permissive_open_weight`: open-weight model with Apache-2.0, MIT, or similarly permissive terms.
- `restricted_open_weight`: open-weight model with special field-of-use, scale, region, or acceptable-use restrictions.
- `non_commercial`: not allowed for shared production infrastructure unless explicitly approved.
- `internal_only`: locally trained or licensed model that cannot be shared externally.

## Default Model Policy

- Prefer permissive open-weight or fully open-source AI models.
- Require a license file, model card, source URL, checksum, and safety notes before deployment.
- Require explicit approval for any model with non-commercial or field-of-use restrictions.
- Keep model cache and inference logs under local data-sovereignty controls.
- Support multiple model families to avoid monoculture.

## Queueing Strategy

Use two queue profiles:

- Kubernetes-native AI queue: Kueue for batch inference, fine-tuning, evaluation, and data processing jobs.
- HPC/training queue: Slurm for MPI-heavy or traditional HPC workflows.

The unified interface should expose:

- Queue position.
- Requested resources.
- Estimated start time.
- Estimated energy and cost.
- Tenant/project allocation.
- Priority and preemption policy.
- Carbon-aware deferral option.

## Serving Strategy

- vLLM for high-throughput OpenAI-compatible text serving.
- SGLang for low-latency and advanced structured/multimodal serving patterns.
- Separate model registry from serving runtime so operators can switch engines.
- Use OCI images and pinned model checksums for reproducibility.
- Use GPU partitions only where supported and tested.

## Safety and Abuse Controls

- Tenant isolation for prompts, outputs, and logs.
- Rate limits and quota budgets.
- Content and security policy hooks.
- Audit logs for model deployment, access, and deletion.
- Red-team tests for each model class before shared service exposure.

## Sovereignty Goals

For developing-world deployments, AI should be useful offline and locally maintainable:

- Keep small language models available for operator support, documentation search, translation, and code assistance.
- Prefer quantized models that can run on modest GPUs or CPUs for disaster-mode operation.
- Keep training and fine-tuning optional; inference is the first operational target.
