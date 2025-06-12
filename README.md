# Better Tor CLI

This project provides a command-line interface (CLI) script to route all your system's network traffic through the Tor network using iptables rules. It is currently designed for Arch Linux systems.

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
- Arch Linux (other distros may require modifications)
- Tor installed and running as a system service
- Root privileges

## GUI (Coming Soon)
A graphical user interface (GUI) is planned for future development. The GUI will use this CLI script under the hood to provide a more user-friendly experience.

---

*Based on the [toriptables3](https://github.com/ruped24/toriptables3) script by Ruped24.*
