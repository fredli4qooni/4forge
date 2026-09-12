# Security Policy

The 4Forge team takes the security of our software and users seriously. As an open-source development tool managing system processes, local proxies, runtimes, and databases, security and transparency are core principles of our architecture.

---

## Supported Versions

Only the latest release of 4Forge receives security patches.

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x (Development) | :white_check_mark: |

---

## Reporting a Vulnerability

If you discover a security vulnerability in 4Forge, please report it privately. **Do NOT open a public GitHub issue.**

To report a vulnerability:
1. Contact the maintainer directly via email: **security@4forge.dev** (or via GitHub Private Vulnerability Reporting once enabled).
2. Include as much information as possible:
   - Description of the vulnerability and its potential impact.
   - Steps to reproduce or proof-of-concept (PoC).
   - The version of 4Forge and Windows OS version tested.
3. We will acknowledge receipt of your report within 48 hours and provide an estimated timeline for remediation.

---

## Security Principles

- **No Telemetry Without Opt-in**: 4Forge collects zero telemetry or analytics by default.
- **Signed Binaries**: Official releases are signed with code-signing certificates to ensure authenticity and integrity.
- **Supply Chain Security**: Dependencies are strictly vetted via `cargo deny check` and `cargo audit`. Software Bill of Materials (SBOM) and checksums are provided with each release.
- **Sandboxed IPC**: Tauri's capability-based security model restricts frontend access to explicitly allowed system commands only.
