import sys
import time

from argparse import ArgumentParser, RawDescriptionHelpFormatter

import httpx

from netspot_control_client import Client as NetspotControlClient
from netspot_control_client.api.status import status_status_all
from netspot_control_client.api.statistics import statistics_ger_data
from netspot_control_client.models import *


def check_results(client):
    """
    This function checks that new packet statistics are received from the service

    :param client:  Netspot Control Client Object
    :return:        True, if everything is okay and False otherwise
    """
    # Requesting latest data result
    print("Requesting latest result...")
    result = statistics_ger_data.sync(client=client, last=1)[0]
    print("Got:", result.to_dict())
    print("Waiting for 2 seconds for more results")
    time.sleep(2)
    print("Requesting more result...")
    results = statistics_ger_data.sync(client=client, time=result.time)
    if len(results) == 0:
        print("Did not get more results, something is wrong")
        return False
    print("Got:")
    for result in results:
        print(result.to_dict())
    print()
    return True


def get_status(client):
    """
    Gets status message from the service and returns configuration IDs

    Can raise error if server does not respond

    :param client:  Netspot Control Client Object
    :return:
        ([running], [stopped], [disabled])
        [running] : list of configurations that are running
        [stopped] : list of configurations that are stopped
        [disabled] : list of configurations that are disabled
    """
    running = []
    stopped = []
    disabled = []

    configurations = status_status_all.sync(client=client)
    for configuration in configurations:
        if configuration.status == ProcessStatus.RUNNING:
            running.append((configuration.id, configuration.name))
        elif configuration.status == ProcessStatus.STOPPED:
            stopped.append((configuration.id, configuration.name))
        elif configuration.status == ProcessStatus.DISABLED:
            disabled.append((configuration.id, configuration.name))
        else:
            raise ValueError("Invalid status")

    return running, stopped, disabled


def test_running_service():
    """
    This function performs a few tests on the running service and
    returns the status of the service using an exit code.

    Returns:
        0   Everything works as it should.
        1   The service does not respond.
        2   Some configurations have been stopped.
        3   Netspots are running, but no new results appear.
        -1  Unexpected error.
    """
    parser = ArgumentParser(
        description=test_running_service.__doc__,
        formatter_class=RawDescriptionHelpFormatter
    )
    parser.add_argument('address', type=str, help='Server address')
    parser.add_argument('port', type=int, help='Server port')
    args = parser.parse_args()

    protocol = 'http'
    address = args.address
    port = args.port
    api_version = 'v1'

    ncc = NetspotControlClient(
        base_url=f'{protocol}://{address}:{port}/{api_version}',
        raise_on_unexpected_status=True,
        timeout=5.0,
        verify_ssl=False
    )

    try:
        print("Requesting status of configurations...")
        running, stopped, disabled = get_status(ncc)
        print("Running configurations:", running)
        print("Stopped configurations:", stopped)
        print("Disabled configurations:", disabled)
        if len(stopped) > 0:
            print("The stopped configurations list should be empty. "
                  "Start the stopped configuration and then try again.")
            return 2
        print()

        if len(running) == 0:
            print("No configuration is active. This is fine, but the service does nothing now.")
            return 0
        if not check_results(ncc):
            return 3

    except httpx.ConnectError as error:
        print(error)
        return 1

    except Exception as e:
        print("Unexpected error:", e)
        return -1

    print("Everything works as it should.")
    return 0


if __name__ == "__main__":
    sys.exit(test_running_service())
