# Data & ML Question Bank

## Data Architecture

### Data Sources & Ingestion
1. What are the data sources? Internal? External? Batch? Stream? CDC?
2. What is the data volume per source? Growth rate? Schema evolution?
3. What is the ingestion latency requirement? Minutes? Hours? Days?
4. What is the data quality at source? Completeness? Accuracy? Consistency?
5. What are the SLAs from upstream? What happens when they're missed?

### Storage & Modeling
6. What is the storage layer? Data lake? Warehouse? Lakehouse? Feature store?
7. What are the table formats? Iceberg? Delta? Hudi? Parquet? ORC?
8. What is the modeling approach? Kimball? Data Vault? Activity Schema? One Big Table?
9. What is the partitioning strategy? Time? Tenant? Geography? Hash?
10. What is the retention policy? Legal? Business? Cost-driven?

### Data Contracts
11. What are the data contracts between producers/consumers? Schema? Semantics? SLAs?
12. How are contracts enforced? Schema registry? CI checks? Runtime validation?
13. What is the versioning strategy? Backward/forward compatibility?
14. How are breaking changes communicated? Migration path?
15. Who owns the contract? Producer? Consumer? Platform team?

## Data Quality

### Validation & Monitoring
16. What are the data quality dimensions? Completeness? Validity? Uniqueness? Consistency? Timeliness? Accuracy?
17. What are the quality rules per dataset? Null checks? Range? Referential integrity? Business rules?
18. Where do checks run? Ingestion? Transformation? Consumption? All three?
19. What happens on failure? Alert? Quarantine? Block? Auto-remediate?
20. What is the quality SLA? % valid rows? Time to detection? Time to resolution?

### Observability
21. What are the data SLIs? Freshness? Volume? Schema? Distribution? Lineage?
22. What is the alerting strategy? Threshold? Anomaly? ML-based?
23. What is the data lineage coverage? Column-level? End-to-end?
24. What is the incident response for data issues? Runbooks? War rooms?
25. How do consumers know about data issues? Status page? Webhooks? API?

## Pipelines & Orchestration

### Pipeline Design
26. What is the orchestration tool? Airflow? Dagster? Prefect? dbt? Custom?
27. What is the DAG structure? Fan-out? Fan-in? Dependencies? Parallelism?
28. What is the idempotency strategy? Reprocessing? Backfills? Late-arriving data?
29. What is the testing strategy? Unit? Integration? Data diffs? Contract tests?
30. What is the deployment strategy? CI/CD? GitOps? Blue-green?

### Reliability
31. What is the retry policy? Exponential backoff? Dead letter queue?
32. What is the SLA for pipeline completion? Alerting on SLA breach?
33. What is the cost monitoring? Per pipeline? Per table? Anomaly detection?
34. What is the capacity planning? Peak vs average? Autoscaling?
35. What is the disaster recovery for pipelines? RTO? RPO? Tested?

## Machine Learning

### Problem Framing
36. What is the ML problem type? Classification? Regression? Ranking? Recommendation? Generation? Anomaly?
37. What is the business metric? What is the proxy ML metric? Correlation?
38. What is the baseline? Heuristic? Existing model? Random? Human?
39. What is the label source? Human? Implicit feedback? Synthetic? Weak supervision?
40. What is the evaluation protocol? Holdout? Time-series split? Cross-validation? Online A/B?

### Data for ML
41. What is the training data? Volume? Quality? Bias? Representativeness?
42. What is the feature engineering? Manual? Automated? Feature store?
43. What is the data leakage prevention? Temporal splits? Group splits?
44. What is the class imbalance strategy? Resampling? Weighting? Threshold tuning?
45. What is the privacy handling? PII? Differential privacy? Federated?

### Model Development
46. What is the experimentation tracking? MLflow? Weights & Biases? Custom?
47. What is the model registry? Versioning? Staging? Promotion criteria?
48. What is the hyperparameter optimization? Grid? Random? Bayesian? Population-based?
49. What is the compute infrastructure? GPU? TPU? Distributed? Spot instances?
50. What is the reproducibility guarantee? Seed? Environment? Data version?

### Model Deployment
51. What is the serving pattern? Online? Batch? Streaming? Edge? Hybrid?
52. What is the latency requirement? p99? Throughput? Cold start?
53. What is the model format? ONNX? TorchScript? TensorRT? Custom?
54. What is the A/B testing framework? Ramp? Shadow? Canary?
55. What is the rollback strategy? Instant? Gradual? Feature flag?

### Monitoring & Maintenance
56. What is the model monitoring? Drift? Performance? Data quality? Concept drift?
57. What is the retraining trigger? Schedule? Drift threshold? Performance drop? Manual?
58. What is the feature store consistency? Online/offline skew?
59. What is the explainability requirement? Regulatory? Debugging? Trust?
60. What is the model governance? Approval? Audit? Compliance? Model cards?

## Privacy & Compliance

61. What regulations apply? GDPR? CCPA? HIPAA? SOX? Industry-specific?
62. What is the data classification? Public? Internal? Confidential? Restricted?
63. What is the consent management? Granular? Revocable? Auditable?
64. What is the right to deletion implementation? Hard? Soft? Archive?
65. What is the data residency requirement? Regions? Sovereignty?

## Team & Operations

66. What is the team structure? Data engineers? ML engineers? Analysts? Scientists?
67. What is the on-call rotation? For pipelines? For models? For data quality?
68. What is the documentation strategy? Data catalog? Data dictionary? Runbooks?
69. What is the knowledge sharing? Office hours? RFCs? Post-mortems?
70. What is the hiring/training plan? Skills gaps? Career paths?