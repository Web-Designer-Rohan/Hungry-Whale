# Security Question Bank

## Threat Modeling

### System Context
1. What is the system boundary? Draw the trust boundaries. Where does trust start/end?
2. What are the actors? Users? Services? Admins? Attackers? Insiders?
3. What are the assets? Data? Keys? Compute? Reputation? Availability?
4. What are the entry points? API? UI? CLI? Webhook? Email? Physical?
5. What are the exit points? Logs? Metrics? Exports? Backups? Integrations?

### STRIDE Analysis
6. **Spoofing**: How can identity be faked? Auth bypass? Token theft? Impersonation?
7. **Tampering**: What data can be modified? In transit? At rest? In memory? In logs?
8. **Repudiation**: Can actions be denied? Audit gaps? Log tampering? Non-repudiation?
9. **Information Disclosure**: What data leaks? Errors? Logs? Side channels? Caching? Backups?
10. **Denial of Service**: What can be exhausted? CPU? Memory? Disk? Network? Quotas? Locks?
11. **Elevation of Privilege**: How can permissions be escalated? Confused deputy? Path traversal? Injection?

### Attack Trees
12. For each high-value asset, draw the attack tree. Root = compromise asset. Leaves = atomic attacks.
13. What are the mitigations per leaf? Prevention? Detection? Response?
14. What is the residual risk after mitigations? Accept? Transfer? Avoid?
15. What are the assumptions? (e.g., "TLS terminates at load balancer", "insiders are trusted")

## Identity & Access Management

### Authentication
16. What auth methods? Password? MFA? SSO? Certificate? Hardware key? Passwordless?
17. What is the session management? JWT? Cookie? Token? Lifetime? Refresh? Revocation?
18. What is the password policy? Length? Complexity? Rotation? Breach check? History?
19. What is the account recovery? Email? SMS? Security questions? Admin? Social?
20. What is the brute force protection? Rate limit? Lockout? CAPTCHA? Alerting?

### Authorization
21. What is the authorization model? RBAC? ABAC? ReBAC? ACL? Custom?
22. What are the roles? Permissions? Hierarchy? Separation of duties?
23. What is the principle of least privilege? Default deny? Just-in-time access?
24. How are permissions assigned? Provisioning? Request/approval? Self-service?
25. How are permissions reviewed? Recertification? Automated? Exception handling?

### Secrets Management
26. What secrets exist? API keys? DB passwords? Certificates? SSH keys? Encryption keys?
27. Where are secrets stored? Vault? HSM? KMS? Env vars? Config files? What are the trade-offs of each?
28. What is the rotation policy? Frequency? Automation? Emergency rotation?
29. What is the access control for secrets? Who can read? Write? Audit?
30. What is the secret scanning? Pre-commit? CI? Runtime? Response?

## Network Security

### Perimeter
31. What is the network topology? VPCs? Subnets? Public/Private? DMZ?
32. What are the ingress/egress rules? Default deny? Explicit allow? Logging?
33. What is the WAF? Rules? Managed? Custom? Blocking vs monitoring?
34. What is the DDoS protection? L3/L4? L7? Always-on? On-demand?
35. What is the TLS termination? Where? Certificate management? Versions? Ciphers?

### Segmentation
36. What is the micro-segmentation? Service mesh? Network policies? Zero trust?
37. What is the east-west encryption? mTLS? IPsec? WireGuard?
38. What is the service-to-service auth? SPIFFE? JWT? mTLS? OPA?
39. What is the DNS security? DNSSEC? DoH? Private zones? Split horizon?
40. What is the network monitoring? Flow logs? IDS/IPS? Anomaly detection?

## Application Security

### Secure Development
41. What is the SDLC security? Threat modeling? Code review? SAST? DAST? SCA? Container scan?
42. What is the dependency management? Pinning? Scanning? Automated updates? License check?
43. What is the secret detection? Pre-commit? CI? Production? Response time?
44. What is the secure coding training? Frequency? Topics? Assessment?
45. What is the security champions program? Ratio? Responsibilities? Authority?

### Runtime Protection
46. What is the runtime protection? WAF? RASP? eBPF? Kernel hardening?
47. What is the container security? Image scanning? Admission control? Runtime monitoring?
48. What is the serverless security? Permissions? Timeout? Concurrency? VPC?
49. What is the API security? Schema validation? Rate limiting? AuthZ at edge?
50. What is the client-side security? CSP? SRI? HSTS? Permissions-Policy?

## Data Protection

### Encryption
51. What data is encrypted at rest? Algorithm? Key management? Key hierarchy?
52. What data is encrypted in transit? TLS 1.3? mTLS? Certificate pinning?
53. What is the key management? KMS? HSM? Cloud provider? Self-managed? Rotation?
54. What is the envelope encryption? DEK? KEK? Master key? Hardware-backed?
55. What is the key access control? Who can encrypt? Decrypt? Rotate? Destroy?

### Data Handling
56. What is the data classification? Public? Internal? Confidential? Restricted? PII? PHI? PCI?
57. What is the data minimization? Collect? Store? Process? Share? Retain? Delete?
58. What is the tokenization/anonymization/pseudonymization strategy?
59. What is the data loss prevention? Discovery? Classification? Monitoring? Blocking?
60. What is the backup encryption? Separate keys? Offline? Immutable? Tested?

## Infrastructure Security

### Cloud Security
61. What is the cloud security posture? CSPM? CNAPP? Benchmarks? (CIS, NIST, PCI)
62. What is the identity perimeter? IAM policies? Conditions? Boundaries? Permissions boundaries?
63. What is the resource configuration? IaC scanning? Drift detection? Remediation?
64. What is the container registry security? Signing? Verification? Vulnerability scanning?
65. What is the serverless security? Least privilege? VPC? Secrets? Monitoring?

### Supply Chain
66. What is the software supply chain? SBOM? Signing? Verification? Provenance? (SLSA)
67. What is the build security? Reproducible? Hermetic? Signed? Attested?
68. What is the dependency trust? Pinning? Review? Alternatives? Internal mirror?
69. What is the artifact promotion? Dev? Staging? Prod? Gates? Approvals?
70. What is the incident response for supply chain compromise?

## Incident Response

### Preparation
71. What is the IR plan? Roles? Runbooks? Communication? Legal? PR?
72. What is the detection capability? SIEM? SOAR? UEBA? Threat intel? Hunting?
73. What is the forensic readiness? Log retention? Immutable? Chain of custody?
74. What is the tabletop exercise cadence? Scenarios? Participants? Lessons learned?
75. What is the war room setup? Physical? Virtual? Tools? Access?

### Response
76. What is the classification? Severity? Types? Escalation matrix?
77. What is the containment strategy? Network? Account? Resource? Data?
78. What is the eradication process? Root cause? Malware removal? Credential rotation?
79. What is the recovery validation? Smoke tests? Data integrity? Monitoring?
80. What is the post-incident process? Blameless postmortem? Action items? Tracking?

### Compliance & Audit
81. What frameworks apply? SOC2? ISO27001? PCI-DSS? HIPAA? FedRAMP? GDPR?
82. What is the audit evidence? Automated? Manual? Continuous? Point-in-time?
83. What is the control testing? Frequency? Sampling? Tooling? Remediation?
84. What is the vendor risk management? Assessment? Monitoring? Contractual?
85. What is the regulatory reporting? Breach notification? Timelines? Authorities?

## Emerging Threats

86. What about AI/ML threats? Prompt injection? Data poisoning? Model extraction? Membership inference?
87. What about supply chain? Dependency confusion? Typosquatting? Malicious maintainer?
88. What about cloud-native? Container escape? Kernel exploit? Side-channel? Cryptojacking?
89. What about identity? OAuth flaws? SAML issues? Session fixation? Token replay?
90. What about quantum? Crypto agility? PQC readiness? Timeline? Inventory?