# Better Tor

This project provides a command-line interface (CLI) script to route all your system's network traffic through the Tor network using iptables rules.

## Features
- **Load Tor iptables rules**: Redirects all outgoing traffic through Tor, anonymizing your network activity.
- **Flush rules**: Restores iptables to their default state, disabling Tor routing.
- **Get public IP**: Shows your current public IP address (useful to verify Tor is working).
- **Refresh circuit**: Changes your Tor circuit to get a new exit IP address.
- **Toggle**: Quickly enable or disable Tor routing.

## Usage
Run the script as root:

```sh
sudo ./better-tor-cli.py --help
```

Example commands:
- `sudo ./better-tor-cli.py --load` — Load Tor iptables rules
- `sudo ./better-tor-cli.py --flush` — Flush rules to default
- `sudo ./better-tor-cli.py --ip` — Show current public IP
- `sudo ./better-tor-cli.py --refresh` — Change Tor circuit
- `sudo ./better-tor-cli.py --toggle` — Toggle Tor routing on/off

## Requirements
- A Linux distro that uses systemd to manage services;
- `tor`;
- `iptables` or `iptables-nft`;
- `sudo` (and a polkit agent if using the GUI version).

## GUI
A graphical user interface (GUI) is now available and is becoming more stable and usable. While still under active development, it provides a friendlier way to use the CLI features. The GUI uses this CLI script under the hood to provide a more user-friendly experience.

---

## ⚠️ Risks, Limitations, and Disclaimer

**Using Tor in this way does not guarantee 100% anonymity or security.**
- Misconfiguration, software leaks, or system vulnerabilities may expose your real IP address or other identifying information.
- DNS leaks, application-level leaks, or improper firewall rules can compromise your privacy.
- Some applications may bypass system iptables rules or use protocols not supported by Tor.
- This script is provided as-is, with no guarantee of security or fitness for any particular purpose.

**You are solely responsible for your own security and privacy.**
- The authors and contributors of this project are **not responsible** for any consequences, damages, or legal issues resulting from the use or misuse of this script.
- Always review and understand the risks before using tools that modify your network stack or claim to provide anonymity.

---

*Based on the [toriptables3](https://github.com/ruped24/toriptables3) script by Ruped24.*
