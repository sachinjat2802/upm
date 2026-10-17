# Security Policy

## Supported Versions

Security updates are actively provided for the following versions of CPM:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1.0 | :x:                |

---

## Reporting a Vulnerability

The CPM team takes the security of our platform and ecosystem extremely seriously.
If you believe you have found a security vulnerability in CPM or any of its bridge transports, please report it to us as described below.

### How to Report
- **Email:** Please send an email to `sachinjat2802@gmail.com` with the subject prefix `[SECURITY VULNERABILITY]`.
- **Details to Include:**
  - Description of the issue and potential impact
  - Step-by-step reproduction instructions or proof-of-concept (PoC)
  - Affected operating system and CPM version
- **Public Disclosure:** Please **DO NOT** disclose the vulnerability publicly or open a public GitHub issue until the issue has been addressed and a patch is released.

---

## Security Architecture Highlights

CPM implements multiple layers of defense-in-depth:
1. **Deterministic Lockfile Verification:** `cpm.lock` mandates SHA-256 cryptographic hashes for every dependency artifact.
2. **Secret Leak Detection:** `cpm scan-secrets` inspects repository trees for high-entropy API keys and leaked cloud credentials before commit or publish.
3. **RBAC Security Policies:** `cpm policy` enforces enterprise allowed/blocked package managers and restricts network telemetry.
4. **Framing Limits:** The stdio bridge transport enforces a strict 64 MB maximum message payload guard (`MAX_MESSAGE_SIZE`) to protect against buffer overflow and denial of service (OOM).
