# Architecture Question Bank

## System Boundaries & Decomposition

### Service Boundaries
1. What are the service boundaries? Draw the lines. What does each service own exclusively?
2. What data does each service own? What data is shared? How is consistency maintained?
3. What happens when a service is unavailable? What degrades? What fails completely?
4. What are the API contracts between services? Who owns the contract? How is versioning handled?
5. What is the deployment topology? How many instances? What are the failure domains?

### Data Flow
6. Trace a request end-to-end. Where does it go? What transforms happen? What state changes?
7. What are the synchronous vs asynchronous paths? Why?
8. Where are the queues? What are the SLAs? What happens when they back up?
9. What data is cached? Where? What's the invalidation strategy?
10. What is the source of truth for each piece of data?

## Scalability & Performance

### Load Characteristics
11. What is the expected QPS? Peak? Daily pattern? Seasonal?
12. What is the latency budget? p50? p99? p999?
13. What is the data volume? Growth rate? Retention policy?
14. What are the hot paths? Cold paths?
15. What is the read/write ratio?

### Capacity Planning
16. What breaks first at 10x load? 100x?
17. What is the scaling strategy? Horizontal? Vertical? Both?
18. What are the bottlenecks? Database? Network? CPU? Memory? External APIs?
19. What is the cost at steady state? At peak? At 10x?
20. What is the auto-scaling policy? Scale-up time? Scale-down time?

## Reliability & Resilience

### Failure Modes
21. Enumerate every component. For each: what happens when it fails? What is the blast radius?
22. What are the single points of failure? How are they mitigated?
23. What is the disaster recovery plan? RTO? RPO?
24. What is the backup strategy? Tested when?
25. What is the chaos engineering practice? Last experiment?

### Observability
26. What are the key SLIs? SLOs? Error budgets?
27. What alerts exist? What is the alert fatigue ratio?
28. How do you debug a production issue? Trace? Logs? Metrics?
29. What is the on-call rotation? Escalation policy?
30. What is the runbook coverage?

## Technology Choices

### Data Stores
31. Why this database? What alternatives considered? Why rejected?
32. What is the schema migration strategy? Zero-downtime?
33. What is the indexing strategy? Query patterns?
34. What is the connection pooling? Timeouts? Retries?
35. What is the read replica strategy? Lag tolerance?

### Infrastructure
36. Why this cloud/provider? Multi-cloud strategy?
37. What is the IaC tool? State management? Drift detection?
38. What is the networking topology? VPCs? Peering? Transit?
39. What is the secret management? Rotation?
40. What is the certificate management? Rotation?

## Security Architecture

41. What is the threat model? STRIDE? PASTA?
42. What is the trust boundary? Where does it start/end?
43. How is authentication handled? Authorization? Audit?
44. What data is encrypted? At rest? In transit? Key management?
45. What is the incident response plan? Last drill?

## Evolution & Maintenance

46. What is the deprecation policy? Sunset process?
47. What is the technical debt budget? How is it tracked?
48. What is the dependency update policy? Security patches?
49. What is the documentation strategy? Living docs? ADRs?
50. What is the knowledge transfer plan? Bus factor?