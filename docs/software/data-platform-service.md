# Data Platform as a Service

The OSDC data platform is an optional sovereign data product layer. It is intended to provide some of the workflow classes associated with Palantir-style platforms, but using open-source tooling and locally controlled infrastructure.

It is not a claim to clone any proprietary product. The intent is to provide the same kind of operational outcome: governed data products, lineage, ontology/business objects, dashboards, notebooks, AI context, data apps, deployment workflows, and audit.

## Default Open Stack

| Capability | Default open-source tooling | Portal workflow |
| --- | --- | --- |
| Ingestion | Airbyte, Meltano, Debezium, Kafka, NATS | Register source, approve connector, land raw dataset. |
| Lakehouse | Ceph RGW or MinIO, Apache Iceberg, Parquet, Nessie or Hive Metastore | Create table, set retention, inspect partitions, manage open table format. |
| Query | Trino, DuckDB, Spark SQL | Run governed SQL, publish certified datasets, enforce quotas. |
| Catalog and lineage | OpenMetadata primary, DataHub optional | Search datasets, assign owner, view lineage, glossary, data contracts. |
| Transformations | dbt Core, SQLMesh, Spark | Versioned SQL/Python transforms with tests and docs. |
| Orchestration | Dagster, Airflow, Argo Workflows | Asset graph, schedules, backfills, retries, run history. |
| Quality | Great Expectations, Soda, dbt tests, OpenMetadata tests | Quality gate, data contract, freshness, completeness, drift. |
| Dashboards | Apache Superset, Metabase, Evidence | Publish governed dashboards and certified datasets. |
| Notebooks | JupyterHub, JupyterLab, optional VS Code Server | Project notebooks with RBAC, secrets policy, and image baseline. |
| Ontology | OpenMetadata glossary, DataHub graph, optional Neo4j or TerminusDB | Business objects, relationships, provenance, and access policy. |
| Data apps | Appsmith, ToolJet, Streamlit, FastAPI, Plotly Dash | Publish operational apps from governed data products. |
| AI context | MLflow, KServe, vLLM, pgvector, Qdrant, OpenMetadata context | Approve datasets for model use, index context, audit prompts and retrieval. |

## User Workflow

```text
Data Platform Console
   |
request data product
   |
register source and owner
   |
land raw data in open lakehouse
   |
attach catalog metadata and lineage
   |
define ontology/business objects
   |
run quality and contract gates
   |
publish SQL, dashboard, notebook, app, or AI context
   |
monitor usage, incidents, freshness, and access
```

## Palantir-Like Capability Map

| Platform class | Open-source OSDC implementation |
| --- | --- |
| Data integration | Airbyte, Meltano, Debezium, Kafka, NATS. |
| Foundry-style object/data model | OpenMetadata glossary/domains, DataHub graph, optional Neo4j/TerminusDB. |
| Lineage and governance | OpenMetadata or DataHub, OpenLineage-compatible pipeline metadata where available. |
| Analytics workspace | Trino, Superset, JupyterHub, DuckDB, dbt Core. |
| Operational apps | Appsmith, ToolJet, Streamlit, FastAPI, Plotly Dash. |
| AI/decision support | MLflow, KServe, vLLM, pgvector/Qdrant, governed retrieval context. |
| Deployment workflow | Forgejo, CI, Harbor, Argo CD or Flux, OpenTofu, OSDC lifecycle audit. |

## Guardrails

- Data products must have an owner.
- Sensitive datasets require classification before publishing.
- Production tables must have quality checks, freshness checks, and lineage.
- Access policies must be enforced at query, dashboard, notebook, and app layers.
- Exports must be logged.
- AI use requires licence, privacy, consent, and lineage review.
- Operational data apps must use approved data products, not ad hoc raw tables.
- Local law, sector rules, and agency data-sharing agreements remain mandatory.

## Portal Data Files

- `data/software/data-platform-services.csv` defines the service stack.
- `data/software/data-products.csv` defines governed data products.
- `data/software/data-pipelines.csv` defines ingestion, transform, and indexing pipelines.
- `data/software/data-ontology-objects.csv` defines business objects and relationships.
- `data/software/data-access-policies.csv` defines access controls.
- `data/software/data-platform-templates.csv` defines starter repo templates.
- `examples/data-platform/` contains generated-template examples for Dagster, dbt, Superset, Argo Workflows, and ontology packs.

## Source Notes

- Apache Iceberg: https://iceberg.apache.org/
- Trino: https://trino.io/
- OpenMetadata: https://docs.open-metadata.org/
- DataHub: https://docs.datahub.com/
- Apache Superset: https://superset.apache.org/
- dbt Core: https://docs.getdbt.com/docs/introduction
- Dagster: https://docs.dagster.io/
- MLflow Model Registry: https://mlflow.org/docs/latest/ml/model-registry/
