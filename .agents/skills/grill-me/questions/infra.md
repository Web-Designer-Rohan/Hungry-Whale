# Infrastructure Question Bank

## Deployment & Releases

### Release Process
1. What is the deployment process? Manual? CI/CD? GitOps?
2. What is the release cadence? Daily? Weekly? Continuous?
3. What is the deployment strategy? Blue-green? Canary? Rolling? Feature flags?
4. How are rollbacks performed? Automated? Time to rollback?
5. What is the release approval process? Who signs off? What gates exist?
6. How are schema/data migrations deployed? Zero-downtime? Expand-contract?

### Environments
7. How many environments exist? Dev? Staging? Prod? Preview?
8. What is the parity between environments? Data? Config? Versions?
9. Who has access to each environment? How is access granted and revoked?
10. What is the promotion path between environments? What gates apply?

## CI/CD

11. What is the CI system? Build time? Cache strategy?
12. What is the pipeline structure? Stages? Parallelism? Failure handling?
13. What is the test strategy in CI? Unit? Integration? E2E? Where do they run?
14. What is the artifact management? Registry? Signing? Immutability? Retention?
15. How are secrets handled in CI? Scope? Rotation?
16. What is the deployment automation? Triggers? Manual approvals? GitOps?

## Observability

### Metrics
17. What are the golden signals? Latency? Errors? Traffic? Saturation?
18. What is the metrics collection? Prometheus? Datadog? Cloud-native? Custom?
19. What is the metrics retention? Granularity? Cost?
20. What are the dashboards? Per service? Per team? On-call view?

### Logging
21. What is the logging stack? Collection? Aggregation? Storage?
22. What is the log structure? Structured? Levels? Correlation IDs?
23. What is the log retention? Compliance? Cost?
24. How is sensitive data handled in logs? Redaction? Scrubbing?

### Tracing
25. What is the tracing coverage? Instrumentation? Sampling rate?
26. What is the trace storage? Retention? Cost?
27. How do you go from alert to trace to log to root cause?

### Alerting
28. What alerts exist? What is the alert fatigue ratio?
29. What is the alert routing? On-call? Escalation? Silence?
30. What is the on-call rotation? Coverage? Handoff? Follow-the-sun?

## Capacity & Performance

31. What is the current utilization? CPU? Memory? Disk? Network?
32. What is the capacity plan? Headroom? Forecasting? Review cadence?
33. What breaks first at 10x load? 100x?
34. What is the autoscaling policy? Triggers? Cooldown? Min/max?
35. What is the load testing practice? Frequency? Tools? Scenarios?
36. What is the performance budget? Latency? Throughput? Per service?

## Networking & Infrastructure

37. What is the infrastructure topology? Regions? AZs? Edge?
38. What is the network design? VPCs? Subnets? Peering? Transit? CDN?
39. What is the DNS strategy? Providers? TTLs? Failover? DNSSEC?
40. What is the load balancing strategy? L4/L7? Health checks? Sticky sessions?
41. What is the IaC tool? Terraform? Pulumi? CloudFormation? State management?
42. What is the drift detection and prevention? Plan checks? Remediation?
43. What is the secret management? Vault? KMS? Rotation? Access control?

## Reliability & Disaster Recovery

44. What is the backup strategy? What is backed up? Frequency? Retention?
45. When were backups last restored and tested?
46. What is the DR plan? RTO? RPO? Failover runbooks? Tested when?
47. What are the single points of failure? How are they mitigated?
48. What is the chaos engineering practice? Game days? Last experiment?
49. What is the incident response process? Severity levels? Postmortems?
50. What is the runbook coverage? Accuracy? Kept up to date?

## Cost & Sustainability

51. What is the monthly infrastructure cost? Per service? Per environment?
52. What is the cost trend? Anomaly detection? Budgets? Alerts?
53. What is the cost optimization? Rightsizing? Reserved instances? Spot?
54. What is the waste? Idle resources? Orphaned storage? Over-provisioning?
55. What is the sustainability plan? Efficiency? Carbon targets?

## Team & Operations

56. What is the infra team structure? SRE? Platform? DevOps? Embedded?
57. What is the toil budget? Automation targets? Measurement?
58. What is the documentation strategy? Runbooks? Architecture diagrams? Ownership?
59. What is the knowledge transfer plan? Bus factor? Onboarding?
60. What is the vendor management? SLAs? Renewals? Exit strategy?
