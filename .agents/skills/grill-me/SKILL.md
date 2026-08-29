---
name: grill-me
description: A relentless interview to sharpen a plan or design through structured questioning.
disable-model-invocation: true
version: 1.0.0
---

# Grill Me

**Sharpen a plan through interview.** This skill conducts a structured, adversarial interview to stress-test plans, designs, architectures, and decisions. It forces clarity by asking the questions you should have asked yourself.

## When to Use

- You have a plan/design but aren't confident it's solid
- You need to validate assumptions before committing resources
- You want to catch blind spots, edge cases, and hidden risks
- You're preparing for a design review or architecture discussion

## The Interview Process

The interview has **three phases**. Each phase targets a different class of failure.

### Phase 1: Clarify the Ask (5-10 min)

**Goal:** Pin down exactly what you're building, for whom, and what "done" looks like.

**Questions:**
1. **What is the one-sentence outcome?** (Not "build X" — "enable Y to do Z")
2. **Who is the user?** (Specific persona, not "developers")
3. **What does success look like?** (Measurable: latency, adoption, cost, error rate)
4. **What is explicitly out of scope?** (List 3+ things you are NOT doing)
5. **What is the hard constraint?** (Deadline, budget, tech stack, team size)

**Exit criterion:** You can write the outcome, user, success metric, and constraints on a sticky note.

### Phase 2: Stress-Test the Plan (15-30 min)

**Goal:** Find the assumptions that, if wrong, invalidate the plan.

**For each major component, ask:**

**Feasibility**
- What's the hardest technical risk? How do you de-risk it *this week*?
- What does the happy path look like? What are the *three* most likely failure modes?
- Have you done this before? If not, who has? What did they learn?

**Interfaces & Dependencies**
- What external systems does this touch? What are their SLAs? What happens when they're down?
- What data flows in/out? What's the schema? Who owns it?
- What happens if the interface changes next month?

**Operational Reality**
- How do you deploy? Roll back? Observe? Debug in production?
- What alerts fire when this breaks? Who wakes up?
- What's the capacity plan? What breaks at 10x load?

**Security & Compliance**
- What data is sensitive? How is it encrypted? Who has access?
- What regulatory requirements apply? (GDPR, SOC2, HIPAA, etc.)
- What's the blast radius if this is compromised?

**Cost & Sustainability**
- What's the monthly bill at steady state? At peak?
- What's the maintenance burden? (On-call, schema migrations, dependency updates)
- What happens if the author leaves?

**Exit criterion:** You have a written list of **risks with mitigations**, not just "we'll figure it out."

### Phase 3: The Pre-Mortem (10 min)

**Goal:** Imagine it's 6 months later and the project failed. Why?

**Questions:**
1. **What was the single biggest cause of failure?**
2. **What warning sign did we ignore?**
3. **What decision seemed reasonable at the time but was wrong?**
4. **What would we do differently if we started over?**

**Output:** A **Pre-Mortem Document** with:
- Top 3 failure scenarios
- Early warning indicators for each
- Contingency plans

## Interview Rules

1. **No answers in questions.** Don't ask "Should we use X?" — ask "What are the criteria for choosing between X and Y?"
2. **One question at a time.** Wait for the answer. Probe.
3. **Follow the thread.** If an answer reveals a new risk, drill into it before moving on.
4. **Document everything.** Every Q&A pair goes in the interview log.
5. **No "we'll figure it out."** Every open question gets an owner and a deadline.

## Output Artifacts

After the interview, you produce:

1. **Interview Log** (`grill-me-<slug>-log.md`) — Full Q&A transcript
2. **Risk Register** (`grill-me-<slug>-risks.md`) — Risks, likelihood, impact, mitigation, owner, deadline
3. **Decision Record** (`grill-me-<slug>-decisions.md`) — Key decisions made, alternatives considered, rationale
4. **Pre-Mortem** (`grill-me-<slug>-premortem.md`) — Failure scenarios and contingencies
5. **Sharpened Plan** (`grill-me-<slug>-plan.md`) — The original plan, revised with interview findings

## Domain-Specific Question Banks

Use the question bank matching your domain:

| Domain | File | Focus |
|--------|------|-------|
| Architecture | `questions/architecture.md` | System design, scalability, reliability |
| Product | `questions/product.md` | User value, metrics, prioritization |
| API Design | `questions/api.md` | Contracts, versioning, errors, auth |
| Data/ML | `questions/data.md` | Pipelines, quality, drift, privacy |
| Security | `questions/security.md` | Threat modeling, compliance, access control |
| Infrastructure | `questions/infra.md` | Deployment, observability, capacity |
| Team/Process | `questions/team.md` | Ownership, onboarding, knowledge sharing |

## Running the Interview

The agent conducts the interview using the question bank for the chosen domain. Pick a mode, name the topic and domain, and the agent runs the session and writes the artifacts.

### Solo (Self-Interview)
Ask the agent to grill you, e.g.:

> Grill me on the new payment service (architecture domain).

The agent selects the matching question bank, walks through Phases 1-3 one question at a time, waits for your answer after each question, probes follow-ups, and writes the artifacts when the interview is complete.

### With a Partner (Async Questionnaire)
Ask the agent to export a questionnaire, e.g.:

> Export a questionnaire for the new payment service (architecture domain).

The agent generates `grill-me-<topic>-questionnaire.md` from the question bank — questions with answer lines, no interview content. Send it to a reviewer. When they return it filled in, the agent compiles the answers into the interview log and the remaining artifacts.

### In a Meeting (Live)
Ask the agent to generate a discussion guide, e.g.:

> Generate a discussion guide for the new payment service (architecture domain).

The agent produces `grill-me-<topic>-guide.md` — phases, timing, and the key questions per phase. Run the session live from the guide; afterwards the agent records the answers and writes the artifacts.

## Output

Artifacts are written to `./grill-me-output/` (created if missing) using the naming convention `grill-me-<slug>-<artifact>.md`. Tell the agent to use a different directory if you prefer.

## Example Session

```
> Grill me on the user-auth-service (architecture domain).

=== GRILL ME: user-auth-service ===
Phase 1: Clarify the Ask
Q1: What is the one-sentence outcome?
> Enable users to securely authenticate and authorize across all our services using OAuth2/OIDC.

Q2: Who is the user?
> Internal services (backend APIs), external developers (public API), end-users (web/mobile apps).

Q3: What does success look like?
> < 50ms p99 latency, 99.99% availability, zero critical CVEs in 12 months.

Q4: What is explicitly out of scope?
> - Social login (Google, GitHub)
> - MFA enforcement (v2)
> - Session management UI
> - Passwordless auth

Q5: What is the hard constraint?
> Must integrate with existing Kong API Gateway. 6-week deadline.

=== Phase 2: Stress-Test ===
[... 50 questions across 6 categories ...]

=== Phase 3: Pre-Mortem ===
Q: What's the single biggest cause of failure?
> Token validation latency cascades under load, causing 504s on all services.

Q: What warning sign did we ignore?
> We didn't load-test the JWKS caching layer.

[... artifacts written to ./grill-me-output/grill-me-user-auth-service-* ...]
```

## Integration with Other Skills

- **subagent-driven-development** — Use interview outputs as plan input for implementation
- The **With a Partner** mode covers async stakeholder review, so no separate questionnaire skill is needed.

## Quality Bar

A "grilled" plan must have:
- [ ] Every Phase 1 question answered in writing
- [ ] Risk register with ≥5 risks, each with mitigation + owner + deadline
- [ ] Decision record with ≥3 key decisions documented
- [ ] Pre-mortem with ≥3 failure scenarios + contingencies
- [ ] No "TBD" or "we'll figure it out" in the sharpened plan

If any checkbox is unchecked, the plan is not ready.

## Philosophy

> "Plans are worthless, but planning is everything." — Eisenhower

The interview *is* the planning. The artifacts are the receipt.