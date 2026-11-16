# Atom Editor

Atom Editor is a SigmaOS component providing a functional domain model for `atom-editor2` with validated state, deterministic storage, and typed errors.

## Description

Atom text editor compatibility layer for SigmaOS

## Domain model

- `TextEditor` — domain entity

## API

- `new()`

## Testing

```sh
cargo test
```

Tests are deterministic, isolated, and self-validating.
