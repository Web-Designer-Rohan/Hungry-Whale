# API Design Question Bank

## Contract Design

### Resources & Endpoints
1. What are the resources? (Nouns, not verbs) Draw the resource graph.
2. What are the relationships? Parent-child? Peer? Many-to-many?
3. What are the standard operations per resource? (CRUD + custom)
4. What is the URL structure? Hierarchical? Flat? Versioned?
5. What are the query parameters for filtering, sorting, pagination?

### Request/Response
6. What is the request/response format? JSON? Protobuf? Avro? Why?
7. What is the envelope structure? Data? Meta? Errors? Links?
8. What are the field naming conventions? camelCase? snake_case? PascalCase?
9. What are the date/time formats? ISO 8601? Unix timestamp? Timezone handling?
10. What are the ID formats? UUID? ULID? Integer? Prefixed?

### Versioning
11. What is the versioning strategy? URL? Header? Media type? Query param?
12. What is the deprecation policy? Notice period? Communication channels?
13. How are breaking vs non-breaking changes defined?
14. What is the compatibility testing strategy?
15. How many versions are supported simultaneously?

## Error Handling

### Error Model
16. What is the error response format? RFC 7807 (Problem Details)? Custom?
17. What are the error categories? Client errors (4xx)? Server errors (5xx)?
18. What are the specific error codes? Machine-readable? Human-readable?
19. What fields are included? Code? Message? Details? Retry-After? Trace ID?
20. How are validation errors structured? Field-level? Global?

### Retry & Resilience
21. Which errors are retryable? Which are not? How does the client know?
22. What is the retry policy? Exponential backoff? Jitter? Max attempts?
23. What is the idempotency strategy? Keys? Headers? Natural idempotency?
24. What is the rate limiting strategy? Headers? Response codes? Quotas?
25. What is the circuit breaker pattern for clients?

## Authentication & Authorization

### Authentication
26. What auth schemes? API keys? JWT? OAuth2? mTLS? Multiple?
27. What is the token format? Opaque? JWT? JWE? Claims?
28. What is the token lifetime? Refresh strategy? Rotation?
29. What is the key management? JWKS? Rotation? Revocation?
30. How is authentication tested? Automated? Manual?

### Authorization
31. What is the authorization model? RBAC? ABAC? ReBAC? Custom?
32. What are the scopes/permissions? Granularity? Hierarchy?
33. How are permissions enforced? Gateway? Service? Both?
34. What is the audit logging for auth decisions?
35. How is authorization tested? Unit? Integration? Chaos?

## Performance & Reliability

### Latency & Throughput
36. What are the latency SLAs? p50? p95? p99? Per endpoint?
37. What are the throughput SLAs? RPS? Burst? Sustained?
38. What is the payload size limit? Request? Response? Streaming?
39. What is the timeout policy? Client? Server? Gateway?
40. What is the compression? gzip? brotli? zstd? When?

### Caching
41. What is cacheable? GET? HEAD? Conditional requests (ETag, Last-Modified)?
42. What is the cache hierarchy? CDN? Gateway? Service? Client?
43. What is the cache key strategy? Vary headers? User-specific?
44. What is the invalidation strategy? TTL? Event-based? Hybrid?
45. What is the stale-while-revalidate policy?

## Observability

### Instrumentation
46. What headers are propagated? Trace ID? Span ID? Correlation ID?
47. What metrics are emitted? Latency? Errors? Rate? Saturation? (RED/USE)
48. What is the logging structure? Structured? Levels? Sampling?
49. What is the distributed tracing coverage? 100%? Sampling rate?
50. What are the synthetic monitors? Health checks? Deep checks?

## Security

### Transport & Data
51. What is the TLS configuration? Versions? Ciphers? HSTS? HPKP?
52. What data is sensitive? PII? Secrets? How is it protected in logs?
53. What is the CORS policy? Origins? Methods? Headers? Credentials?
54. What is the CSP? HSTS? X-Frame-Options? Referrer-Policy?
55. What is the input validation? Schema? Sanitization? Size limits?

### Abuse Prevention
56. What is the DDoS protection? WAF? Rate limiting? Geo-blocking?
57. What is the bot detection? CAPTCHA? Challenge? Behavioral?
58. What is the API abuse detection? Anomaly? ML? Rules?
59. What is the incident response for API abuse?
60. What is the penetration testing cadence? Scope?

## Developer Experience

### Documentation
61. What is the documentation format? OpenAPI/Swagger? AsyncAPI? Custom?
62. Is documentation generated from code? Contract tests? Manual?
63. What is the interactive docs? Swagger UI? Redoc? Custom?
64. What are the code samples? Languages? Frameworks? SDKs?
65. What is the changelog? Format? Communication?

### Onboarding
66. What is the getting started guide? Time to first call?
67. What is the sandbox environment? Data? Reset? Limits?
68. What is the SDK strategy? Generated? Hand-written? Languages?
69. What is the support channel? Slack? Email? Forum? SLA?
70. What is the deprecation communication? Channels? Timeline?