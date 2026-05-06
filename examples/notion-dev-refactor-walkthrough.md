# Notion Dev Refactor Guard Walkthrough

This note is the quickest way to read the extra review model in `notion-dev-refactor-guard`.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 148 | ship |
| stress | diagnostic quality | 236 | ship |
| edge | review cost | 187 | ship |
| recovery | safe rewrite | 180 | ship |
| stale | change width | 183 | ship |

Start with `stress` and `baseline`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

The next useful expansion would be a malformed fixture around diagnostic quality and safe rewrite.
