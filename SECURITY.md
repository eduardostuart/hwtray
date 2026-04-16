# Security Policy

## Reporting a Vulnerability

If you believe you've found a security issue in this project, please **do not open a public issue**.

Instead, email the maintainer at the address listed on the [author's page](https://s.tuart.dev). If private vulnerability reporting is enabled for the repository, you may also submit a private advisory from the **Security** tab on GitHub.

Please include:

- A description of the issue and its potential impact
- Steps to reproduce (or a proof of concept, if applicable)
- The affected version (see `tauri.conf.json` or the release tag)

You should expect an initial response within a few days. This is a small personal project, so response times are best-effort.

## Supported Versions

Only the latest release receives security fixes. Please upgrade before reporting issues against older versions.

## Scope

In scope:

- The Tauri desktop app (Rust backend, Vue frontend, IPC surface)
- Local config/telemetry file handling
- HTTP client behavior against HomeWizard devices on the local network
- mDNS discovery

Out of scope:

- Issues in the HomeWizard firmware or its local API (report those to HomeWizard)
- Issues in upstream dependencies unless this project's usage makes them exploitable
- Social engineering, physical attacks, or issues requiring local root access
