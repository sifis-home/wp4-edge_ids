# Integration Tests

There are two tests in this folder. One is suitable for testing the existing netspot control setup, and the other can be used to test the container more thoroughly.

## Test Running Service

File: `test-running-service.py`

This can be used to check the correct functionality of the existing netspot control installation. Please note that the instructions below assume the reader uses Debian Linux. Adapt the instructions to match your system.

### Setup

We need to install venv module for python and create a virtual environment for the project. Then, we continue activating the environment and installing the required python modules with pip.

Open your favorite terminal and let's start by installing venv:

```bash
sudo apt install python3-venv
```

Now let's go to the test folder and enable the virtual environment

```bash
cd /path/to/wp4-edge_ids/test/python
python3 -m venv venv
source venv/bin/activate
```

We install the necessary Python modules and print the program instructions with the -h argument.

```bash
python -m pip install -r requirements.txt
python test-running-service.py -h
```

Output:

```
usage: test-running-service.py [-h] address port

    This function performs a few tests on the running service and
    returns the status of the service using an exit code.

    Returns:
        0   Everything works as it should.
        1   The service does not respond.
        2   Some configurations have been stopped.
        3   Netspots are running, but no new results appear.
        -1  Unexpected error.
    

positional arguments:
  address     Server address
  port        Server port

optional arguments:
  -h, --help  show this help message and exit

```

We can check, for example, the local netspot control on port 2080 by issuing the command:

```bash
python test-running-service.py localhost 2080
```

### Simulating errors

#### Exit Code 1

This is easy. we can stop the netspot control service or simply give the wrong address or port as arguments.

#### Exit Code 2

We can use a browser to go to the netspot control developer interface and create a new stopped configuration or stop an existing one. After this, running the test returns exit code 2 as a sign that some configuration is not running but has not been disabled.

#### Exit Code 3

This is a bit more challenging, but we can kill the netspot processes inside to simulate it crashing. We can do this in the container by listing processes using the `ps` command and then using the `kill` command to stop the netspot processes. The `ps` command is not directly available in the container but we can install it.

Accesing bash shell in the container:

```bash
docker exec -it netspot_control /bin/bash
```

Updating packages and installing the `ps` in the container:

```bash
apt update
apt install procps
```

Listing processes:

```bash
ps -ef
```

Output example:

```
UID          PID    PPID  C STIME TTY          TIME CMD
root           1       0  0 13:36 ?        00:00:00 netspot_control
root          15       1  0 13:36 ?        00:00:00 netspot run -c /tmp/netspot_1.toml
root          26       0  0 13:36 pts/0    00:00:00 /bin/bash
root         362      26  0 13:38 pts/0    00:00:00 ps -ef

```

Look for netspot process and use its PID on the next `kill` command

```bash
kill 15
```

We can now run the test and it returns exit code 3 because no more new results appear.

## Test Container

File: `test-container.py`

This is a work in progress and for now the script just starts the test container, checks that there is a response and then closes it.