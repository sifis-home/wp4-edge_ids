# Netspot Alarm Check

This program checks if the Netspot Control service has any new alarm messages available. If multiple alarms are
received, then the program shows the results for the one with the highest probability value.

Running the application needs two arguments. The address for the server and port number the server is listening to.

For example, connecting to the localhost server on port 2000:
```bash
python3 check.py 127.0.0.1 2000
```

By default, the application asks the server to send all alarms since the last time the program was run. Alarms request
is also limited to the 50 most recent available since the previous run. This behavior can be changed by giving an
optional argument `--within <minutes>`, where `<minutes>` is the number of minutes from the current time backward, we
request the server to send alarms.

The program exits with code 1 if alarms are available, with 0 if not, and 2 on errors.

## Setup

The example client requires the `Requests` Python package: https://pypi.org/project/requests/

You can use the `requirements.txt` to install required packages to your system or into the virtual environment.

```bash
pip install -r requirements.txt
```

## Example commands

**Print help**

```bash
python3 check.py --help
```

**Get new alarm from 127.0.0.1 port 2000**

```bash
python3 check.py 127.0.0.1 2000
```

**Get new alarm that is within 20 minutes from the current time **

```bash
python3 check.py 127.0.0.1 2000 --within 20
```

**Get new alarm with custom device name**

```bash
python3 check.py 127.0.0.1 2000 --device "Smart TV"
```

Example output:

```json
{"Device":"Smart TV","Statistic":"R_SYN","Status":"UP_ALERT","Probability":0.75,"Time:":1667472705656570000}
```

