#!/usr/bin/env python3
"""
The Better Ecosystem Tor CLI script (only supports arch for now).
The GUI (still in early development) will use this script under the hood.

Based on the toriptables3 script by Ruped24:
https://github.com/ruped24/toriptables3
"""

from subprocess import call, check_call, CalledProcessError, getoutput
from os.path import isfile, basename
from os import devnull, geteuid
from sys import exit, stdout, stderr
from atexit import register
from argparse import ArgumentParser, RawTextHelpFormatter
from json import load
from urllib.request import urlopen
from urllib.error import URLError
from time import sleep
from shutil import which

class TorIptables:

  def __init__(self):
    self.local_dnsport = "53"
    self.virtual_net = "10.0.0.0/10"
    self.local_loopback = "127.0.0.1"
    self.local_loopback_ipv6 = "::1"
    self.non_tor_net = ["192.168.0.0/16", "172.16.0.0/12"]
    self.non_tor_net_ipv6 = ["fc00::/7", "fe80::/10", "fec0::/10", "ff00::/8"]
    self.non_tor = ["127.0.0.0/9", "127.128.0.0/10", "127.0.0.0/8"]
    self.non_tor_ipv6 = ["::1/128", "::ffff:0:0/96"]
    try:
        self.tor_uid = getoutput("id -ur tor").strip()
        if not self.tor_uid:
            print("\\033[91m[!] Failed to get UID for 'tor' user. Make sure 'tor' user exists and 'id -ur tor' command works.\\033[0m")
            exit(1)
    except Exception as e:
        print(f"\\033[91m[!] Error getting 'tor' user UID: {e}\\033[0m")
        exit(1)
    self.trans_port = "9040"
    self.tor_config_file = '/etc/tor/torrc'
    self.torrc = r'''
VirtualAddrNetwork %s
AutomapHostsOnResolve 1
TransPort %s
DNSPort %s
''' % (self.virtual_net, self.trans_port, self.local_dnsport)

  def flush_iptables_rules(self):
    call(["iptables", "-F"])
    call(["iptables", "-t", "nat", "-F"])
    call(["ip6tables", "-F"])
    call(["ip6tables", "-t", "nat", "-F"])

  def load_iptables_rules(self):
    self.flush_iptables_rules()
    self.non_tor.extend(self.non_tor_net)

    @register
    def restart_tor():
      fnull = open(devnull, 'w')
      try:
        tor_restart = check_call(
            ["systemctl", "restart", "tor"],
              stdout=fnull, stderr=fnull)

        if tor_restart == 0:
          print(" {0}".format(
              "[\033[92m+\033[0m] Anonymizer status \033[92m[ON]\033[0m"))
          self.get_ip()
      except CalledProcessError as err:
        print("\033[91m[!] Command failed: %s\033[0m" % ' '.join(err.cmd))

    call(["iptables", "-I", "OUTPUT", "!", "-o", "lo", "!", "-d",
          self.local_loopback, "!", "-s", self.local_loopback, "-p", "tcp",
          "-m", "tcp", "--tcp-flags", "ACK,FIN", "ACK,FIN", "-j", "DROP"])
    call(["iptables", "-I", "OUTPUT", "!", "-o", "lo", "!", "-d",
          self.local_loopback, "!", "-s", self.local_loopback, "-p", "tcp",
          "-m", "tcp", "--tcp-flags", "ACK,RST", "ACK,RST", "-j", "DROP"])

    call(["iptables", "-t", "nat", "-A", "OUTPUT", "-m", "owner", "--uid-owner",
          "%s" % self.tor_uid, "-j", "RETURN"])
    call(["iptables", "-t", "nat", "-A", "OUTPUT", "-p", "udp", "--dport",
          self.local_dnsport, "-j", "REDIRECT", "--to-ports", self.local_dnsport])

    for net in self.non_tor:
      call(["iptables", "-t", "nat", "-A", "OUTPUT", "-d", "%s" % net, "-j",
            "RETURN"])

    call(["iptables", "-t", "nat", "-A", "OUTPUT", "-p", "tcp", "--syn", "-j",
          "REDIRECT", "--to-ports", "%s" % self.trans_port])

    call(["iptables", "-A", "OUTPUT", "-m", "state", "--state",
          "ESTABLISHED,RELATED", "-j", "ACCEPT"])

    for net in self.non_tor:
      call(["iptables", "-A", "OUTPUT", "-d", "%s" % net, "-j", "ACCEPT"])

    call(["iptables", "-A", "OUTPUT", "-m", "owner", "--uid-owner", "%s" % self.tor_uid, "-j", "ACCEPT"])
    call(["iptables", "-A", "OUTPUT", "-j", "REJECT"])

    call(["ip6tables", "-A", "OUTPUT", "-m", "owner", "--uid-owner", "%s" % self.tor_uid, "-j", "ACCEPT"])
    call(["ip6tables", "-A", "OUTPUT", "-j", "REJECT"])

  def get_ip(self):
    print(" {0}".format(
        "[\033[92m*\033[0m] Getting public IP, please wait..."))
    retries = 0
    my_public_ip = None
    while retries < 12 and not my_public_ip:
      retries += 1
      try:
        my_public_ip = load(urlopen('https://check.torproject.org/api/ip'))['IP']
      except URLError:
        sleep(5)
        print(" [\033[93m?\033[0m] Still waiting for IP address...")
      except ValueError:
        break
    pass
    if not my_public_ip:
      my_public_ip = getoutput('wget -qO - ident.me')
    if not my_public_ip:
      exit(" \033[91m[!]\033[0m Can't get public ip address!")
    print(" {0}".format("[\033[92m+\033[0m] Your IP is \033[92m%s\033[0m" % my_public_ip))

  def is_iptables_loaded(self):
    """Check if Tor iptables rules are loaded by looking for a specific rule."""
    from subprocess import getoutput
    rules = getoutput('iptables -t nat -S')
    return f"--to-ports {self.trans_port}" in rules


# Check if tor is installed, else instruct user to install
if which('tor') is None:
    # Detect package manager
    pkgman = None
    if which('pacman'):
        pkgman = 'pacman'
        install_cmd = 'sudo pacman -S tor'
    elif which('apt'):
        pkgman = 'apt'
        install_cmd = 'sudo apt update && sudo apt install tor'
    elif which('rpm'):
        pkgman = 'rpm'
        install_cmd = 'sudo rpm -i tor'
    else:
        install_cmd = 'Please install tor using your system package manager.'
    print(f"\033[91m[!] 'tor' is not installed.\033[0m")
    print(f"\033[93mTo install tor, run:\033[0m\n  {install_cmd}")
    exit(1)


if __name__ == '__main__':
  if geteuid() != 0:
    exit("You need to run this script as root!")

  parser = ArgumentParser(
      description='\033[1mTor Iptables script for loading and unloading iptables rules\033[0m',
      formatter_class=RawTextHelpFormatter)
  parser.add_argument('-l',
                      '--load',
                      action='store_true',
                      help='\033[92mLoad tor iptables rules\033[0m')
  parser.add_argument('-f',
                      '--flush',
                      action='store_true',
                      help='\033[91mFlush iptables rules to default\033[0m')
  parser.add_argument('-r',
                      '--refresh',
                      action='store_true',
                      help='\033[94mChange the circuit and get a new IP\033[0m')
  parser.add_argument('-i',
                      '--ip',
                      action='store_true',
                      help='\033[96mOutput the current public IP address\033[0m')
  parser.add_argument('-t',
                      '--toggle',
                      action='store_true',
                      help='\033[93mToggle Tor iptables rules ON/OFF\033[0m')
  args = parser.parse_args()

  try:
    load_tables = TorIptables()
    if isfile(load_tables.tor_config_file):
      if not 'VirtualAddrNetwork' in open(load_tables.tor_config_file).read():
        with open(load_tables.tor_config_file, 'a+') as torrconf:
          torrconf.write(load_tables.torrc)

    if args.load:
      load_tables.load_iptables_rules()
    elif args.flush:
      load_tables.flush_iptables_rules()
      print(" {0}".format(
          "[\033[93m!\033[0m] Anonymizer status \033[91m[OFF]\033[0m"))
    elif args.ip:
      load_tables.get_ip()
    elif args.refresh:
      call(['kill', '-HUP', '%s' % getoutput('pidof tor')])
      load_tables.get_ip()
    elif args.toggle:
      if load_tables.is_iptables_loaded():
        load_tables.flush_iptables_rules()
        print(" {0}".format(
            "[\033[93m!\033[0m] Anonymizer status \033[91m[OFF]\033[0m"))
      else:
        load_tables.load_iptables_rules()
      exit(0)
    else:
      parser.print_help()
  except Exception as e:
    print(f"\033[91m[!] An unexpected error occurred: {e}\033[0m")
