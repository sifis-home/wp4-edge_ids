# Simple Netspot Control Client

This folder contains an example client that allows us to check the configuration status and receive statistics from the running netspot_control server.

## Setup

The example client requires the `Requests` Python package: https://pypi.org/project/requests/

You can use the `requirements.txt` to install required packages to your system or into the virtual environment.

```bash
pip install -r requirements.txt
```

## Example commands

**Print help**

```bash
python3 client.py --help
```

**Show status from local netspot_control container**

```bash
python3 client.py -s
```

**Show status, alarm and data messages**

```bash
python3 client.py -s -ga -gd
```

**Follow data messages from server at http://195.148.65.40:3000/**

```bash
 python3 client.py -a http://195.148.65.40:3000/ -fd
```

