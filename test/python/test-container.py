import docker
import httpx
import sys
import time

from docker.errors import DockerException
from netspot_control_client import Client as NetspotControlClient
from netspot_control_client.api.status import status_status_all
from netspot_control_client.models import *

# Using this image to run a container for testing
docker_image = "netspot_control:v1.0.2"

# Using this port to communicate with netspot_control in the container
netspot_control_port = 2080

# Name for the test container
test_container_name = "netspot_control_test"


def check_docker_image(client, tag):
    """
    Check if docker has image with *tag* available.

    :param client:  Docker client
    :param tag:     Search for image with this tag
    :return:        True if found, False otherwise
    """
    for image in client.images.list():
        if tag in image.tags:
            return True
    return False


def wait_for_status(client, timeout=5.0):
    """
    Calls the status endpoint until it gets a response. This is used
    to ensure that the test container is ready for use.

    Raises an TimeoutError if the given timeout is reached.

    :param client:      Netspot control client object
    :param timeout:     Timeout in seconds
    """
    timeout = time.time() + timeout
    while time.time() < timeout:
        try:
            status = status_status_all.sync(client=client)
            if status[0].status == ProcessStatus.RUNNING:
                return

        # We are expecting these errors until the server in the container starts
        except (httpx.ReadError, httpx.RemoteProtocolError):
            pass

        # Let's not waste CPU time unnecessarily
        time.sleep(0.01)

    raise TimeoutError("We did not receive a response from the test container in time")


def main():
    """
    The main function runs the test container, executes the tests, and
    stops the container at the end.

    :return:    0 if all tests passed
                1 if anything goes wrong
    """

    # Using Docker client configured from environment variables
    try:
        client = docker.from_env()
    except DockerException as err:
        print(err)
        return 1

    # Is the required image available?
    if not check_docker_image(client, docker_image):
        print(f"Required image {docker_image} is not available.")
        print("Download it from https://github.com/sifis-home/wp4-edge_ids/releases")
        return 1

    # Run the test container
    print(f"Starting test container from {docker_image} as {test_container_name}")
    container = client.containers.run(
            auto_remove=True,
            detach=True,
            environment=[f"ROCKET_PORT={netspot_control_port}", "SHOW_NETSPOT_MESSAGES=1"],
            image=docker_image,
            name="netspot_control_test",
            ports={f"{netspot_control_port}/tcp": netspot_control_port}
        )

    try:
        # Making a client object to communicate with netspot control
        ncc = NetspotControlClient(
            base_url=f'http://localhost:{netspot_control_port}/v1',
            raise_on_unexpected_status=True,
            timeout=5.0,
            verify_ssl=False
        )
        print("Waiting for the test container to run correctly...")
        wait_for_status(ncc)

        print("TODO: Testing with the test container")

    finally:
        # Stop the test container
        print("Stopping and removing the test container")
        container.stop()

    # Test was successful
    return 0


if __name__ == "__main__":
    sys.exit(main())
