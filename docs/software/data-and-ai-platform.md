# Data and AI Platform

The data and AI platform turns the sovereign cloud into a useful public-sector data environment. It should start small, with clear data governance and model-licence controls.

## Data Services

| Proprietary class | Open-source equivalent | UI workflow |
| --- | --- | --- |
| Managed SQL | CloudNativePG, Percona Operator, MariaDB Operator | Create database, backup, restore, rotate secret. |
| NoSQL | ScyllaDB, Cassandra, FoundationDB, FerretDB | Create collection/keyspace, quota, backup. |
| Cache | Valkey | Create cache, memory quota, eviction policy. |
| Queues and events | NATS, RabbitMQ, Kafka, Knative Eventing | Create queue/topic, view lag, retention. |
| Streaming | Kafka, Redpanda, Apache Pulsar | Create topic, view consumer lag, approve retention. |
| Query data lake | Trino, Iceberg, Ceph/MinIO, Superset | Query datasets, approve access, publish dashboard. |
| Warehouse | ClickHouse, Apache Doris, PostgreSQL | Analytics workspace and retention. |
| Catalogue | OpenMetadata, DataHub, Apache Atlas | Datasets, ownership, lineage. |
| Pipelines | Airflow, Argo Workflows, Dagster | Schedule pipelines and view failures. |

## AI Services

| Proprietary class | Open-source equivalent | UI workflow |
| --- | --- | --- |
| ML workspaces | Kubeflow, MLflow, Ray, KServe | ML workspace and experiment tracking. |
| Model endpoints | vLLM, SGLang, llama.cpp, Ollama for small edge | Request endpoint, model licence, quota, energy cost. |
| GPU queues | Kueue, Slurm | Fair-share queue position, estimate start, energy class. |
| Model registry | MLflow + Harbor OCI artifacts | Approved models and deployment state. |
| Vector search | Qdrant, Milvus, Weaviate, pgvector | Embedding store and quota. |
| Notebooks | JupyterHub | Research notebooks. |
| Labelling | Label Studio | Annotation projects. |
| AI policy | OPA + model licence scanner + dataset policy | Model access approval. |

## AI Caution

Open-weight is not always open source. The UI must show model licence class, approved users, data retention, GPU cost, energy cost, and whether a model may be used for government workloads.

## Source Notes

- NATS: https://nats.io/
- Apache Kafka: https://kafka.apache.org/intro
