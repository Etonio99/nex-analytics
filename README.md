# <img width="28" height="28" alt="app-icon" src="https://github.com/user-attachments/assets/b036e8fe-bbc8-4770-b459-a2db87fe7363" /> Nex Analytics

A desktop application for exporting reports from [NexHealth](https://nexhealth.com) practices. Built with [Tauri](https://tauri.app) (Rust + React).

> **Disclaimer:** This is an independent, open source project and is **not** an official NexHealth product. It is not affiliated with, endorsed by, or supported by NexHealth.

## What It Does

Nex Analytics walks you through a step-by-step process to pull data from NexHealth practices and export it as a formatted Excel (.xlsx) report.

**Current reports:**

- **Available Slots** — Export available appointment slot counts across selected locations for a given date range and appointment type.

## Download

Pre-built installers are available on the [Releases](https://github.com/Etonio99/nex-available-slots-report/releases) page. Download the appropriate installer for your platform (macOS, Windows, or Linux) and run it. We also have a Google Doc on the installation process [here](https://docs.google.com/document/d/1pRn-u8xyxDq_oh1KilF7SvBKnl27MgreTsOWl7USqlY/edit?usp=sharing).

## How to Use

1. **Launch** the app and select the report you want to run from the home screen.
2. **Enter your API key** — the app will validate it against the NexHealth API before proceeding.
3. **Provide Information** - Fill out the required pages to let the tool know of which analytics you are looking to obtain.
4. **Confirm** your selections and let the app generate your report.
5. **Open or reveal** the generated Excel file when processing is complete.

Your API key and subdomain selections are saved locally so you don't need to re-enter them each time.

## API Keys

### How do I get one?

Sign up for a NexHealth developer account at [developers.nexhealth.com/signup](https://developers.nexhealth.com/signup). You may also want to check if your organization already has an API key you can use.

### Getting access to a practice

Receiving an API key does not automatically grant you access to any practice's data. An administrator at the practice must authorize your access by emailing [developers@nexhealth.com](mailto:developers@nexhealth.com). Their email can follow this template:

> We authorize {your organization name} to access our practice data through the NexHealth Synchronizer. Our practice address is {practice address}.

Once the NexHealth support team confirms the request, access will be granted.

### How is my API key stored?

Your API key is stored in your operating system's secure credential store — **Apple Keychain** on macOS, **Windows Credential Manager** on Windows, and **Secret Service** on Linux. It is never written to disk in plain text.

## Subdomains

A subdomain is a formatted identifier for a practice used when making API requests. It is typically the practice name in lowercase with spaces replaced by dashes — for example, `My Dental Office` becomes `my-dental-office`.

To find a practice's subdomain, log into [developers.nexhealth.com](https://developers.nexhealth.com), search for the practice in the top-left corner, and view the subdomain on their overview page.

## Development

### Prerequisites

- [Node.js](https://nodejs.org) 18+
- [Rust](https://rustup.rs) toolchain + Cargo
- Tauri CLI prerequisites for your platform — see the [Tauri setup guide](https://tauri.app/start/prerequisites/)

### Getting started

```bash
# Install JS dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Build

```bash
# Bundle a production desktop app
npm run tauri build
```

### Code quality

```bash
npm run lint        # Lint TypeScript/React
npm run format      # Lint + format Rust and JS
```

## Contributing

Contributions are welcome. Open an issue or pull request on [GitHub](https://github.com/Etonio99/nex-analytics).
