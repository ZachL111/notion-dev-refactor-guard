# notion-dev-refactor-guard

`notion-dev-refactor-guard` is a compact Rust repository for developer tools, centered on this goal: Build a Rust toolkit that studies refactor behavior through layout fixtures, with stable geometry snapshots and bounded memory input sets.

## Why I Keep It Small

The point is to make a small domain rule concrete enough that a reader can change it and immediately see what broke.

## Notion Dev Refactor Guard Review Notes

The first comparison I would make is `diagnostic quality` against `change width` because it shows where the rule is most opinionated.

## Included Behavior

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/notion-dev-refactor-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `diagnostic quality` and `change width`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Internal Model

The core code exposes a scoring path and the added review layer uses `signal`, `slack`, `drag`, and `confidence`. The domain terms are `change width`, `diagnostic quality`, `review cost`, and `safe rewrite`.

The Rust implementation avoids hidden state so fixture changes are easy to reason about.

## Try It Locally

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Validation

The verifier is intentionally local. It should fail if the fixture score math, lane assignment, or language-specific test drifts.

## Scope

No external service is required. A deeper version would add more negative cases and a clearer boundary around invalid input.
