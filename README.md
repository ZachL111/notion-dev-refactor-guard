# notion-dev-refactor-guard

`notion-dev-refactor-guard` explores developer tools in Rust. The repository keeps the core rule set compact, then surrounds it with examples that show how the decisions move.

## Notion Dev Refactor Guard Notes

The quickest review path is the verifier first, then the fixtures, then the operations note. That order makes it easy to see whether the code, data, and explanation still agree.

## Why This Exists

This project keeps the domain idea close to the tests. That makes it useful as a reference implementation, a small experiment, or a starting point for a more specialized tool.

## Code Tour

- `src`: primary implementation
- `tests`: verification harness
- `fixtures`: compact golden scenarios
- `examples`: expanded scenario set
- `metadata`: project constants and verification metadata
- `docs`: operations and extension notes
- `scripts`: local verification and audit commands
- `Cargo.toml`: Rust package metadata

## Feature Notes

- Includes extended examples for safe defaults, including `surge` and `degraded`.
- Documents repeatable output tradeoffs in `docs/operations.md`.
- Runs locally with a single verification command and no external credentials.
- Stores project constants and verification metadata in `metadata/project.json`.
- Adds a repository audit script that checks structure before running the language verifier.

## Implementation Notes

The core is a scoring model over demand, capacity, latency, risk, and weight. That keeps code shape, diagnostics, and safe defaults in one explicit decision path. The threshold is 166, with risk penalty 5, latency penalty 2, and weight bonus 6. The Rust code keeps ownership and data movement plain, which makes the tests useful for checking both behavior and API shape.

## Local Setup

The only required setup is the local Rust toolchain. After cloning, stay in the repo root so fixture paths resolve correctly.

## Example Scenarios

The examples are meant to be readable before they are exhaustive. They cover enough variation to show how latency and risk can pull a decision below the threshold.

## Try It

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

This runs the language-level build or test path against the compact fixture set.

## Tests

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/audit.ps1
```

The audit command checks repository structure and README constraints before it delegates to the verifier.

## Roadmap

- Add a comparison mode that shows how decisions change when one signal is adjusted.
- Add a loader for `examples/extended_cases.csv` and promote selected cases into the language test suite.
- Add a short report command that prints the score breakdown for a single scenario.
- Add one more developer tools fixture that focuses on a malformed or borderline input.

## Boundaries

The repository favors determinism over breadth. It does not pull live data, keep secrets, or depend on network access for verification.
