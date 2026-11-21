# SigmaOS-atom-editor

SigmaOS integration component for **Atom editor**.

## Commands

```bash
sigmaos-atom-editor install   # install package + deploy defaults
sigmaos-atom-editor deploy    # copy config templates to $HOME
sigmaos-atom-editor status    # binary/package/config report
sigmaos-atom-editor reset     # remove deployed config
sigmaos-atom-editor open      # launch the application
```

## Design

- Shell-based component manager (Omarchy-style per-app repos)
- Secure defaults; no secrets stored
- Deterministic `test.sh` with isolated `$HOME`
