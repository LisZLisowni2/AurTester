# Security Policy

## Supported Versions

We actively maintain and support the security of the latest release of `aur-sandbox-tester`. If a vulnerability is found, we strongly recommend upgrading to the latest version as soon as a patch is made available.

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

**Please do not open a public GitHub issue for security vulnerabilities.** Public disclosure risks exposing users before a patch can be developed and deployed.

If you discover a security vulnerability within this project (such as a sandbox escape, a pcap bypass, or a remote code execution vulnerability on the host), please report it to us confidentially using one of the following methods:

- **Email**: Send a detailed description of the issue to **rafalkozikowski735@gmail.com**
- **GitHub Private Vulnerability Reporting**: If enabled on this repository, you can submit a report privately via the "Security" tab -> "Vulnerability reporting".

### What to Include in Your Report

To help us validate and fix the vulnerability as quickly as possible, please include:
1. A clear, descriptive title.
2. A detailed description of the vulnerability and its potential impact.
3. Steps to reproduce the issue (a Proof of Concept or a minimal `PKGBUILD` that triggers the bypass).
4. Any relevant system logs or network captures.

## Our Process

Upon receiving a valid security report, we pledge to:
1. Acknowledge the receipt of your report within 48 hours.
2. Investigate the issue swiftly and keep you updated on our progress.
3. Work on a fix or mitigation strategy.
4. Issue a new release containing the security patch.
5. Provide credit to the reporter in our release notes (if desired).

Thank you for helping keep the AUR Tester community safe!
