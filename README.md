# WP4 netspot

[![LICENSE][license badge]][license] [![Actions Status][actions badge]][actions] [![CodeCov][codecov badge]][codecov]

This repository is for implementing SIFIS-Home WP4 Multi-Level Analysis using the SPOT algorithm. The solution is planned to run as a docker container where the controlling program provides HTTP REST API for other services. The controlling service is designed to run netspot programs and collect data to the cache database.

Below is initial plans for the service.

![Initial plans for the solution](docs/NetSpotControl.png)

## Project folders

* *db – database location, when developing, can be changed from the .env file*
* docs – Documentation and plans
* examples – Example clients
  * python – Simple example for making request with Python 

* migrations – Diesel migration scripts for the database schema
* src – Netspot Control service source files
  * api_v1 – Source files for the HTTP API version 1.x.x (and also 0.x.x while still in development)
  * state – Source files for shared state
  * structures – Source files for structures used between components

* static – Static files served by the Netspot Control server when run

## Netspot

We are planning to use the following Go Implementation of SPOT
https://github.com/asiffer/netspot

Python version of the algorithm is also available:
https://github.com/Amossys-team/SPOT

Project Page:
https://asiffer.github.io/netspot/

## Docker Compose

The project comes with a `docker-compose.yml` file. You can modify it according to your system and wishes. By default, the file compiles the docker image directly from the source codes, but you can comment out the `build` line and remove the comment from the `image` line, which will use the prebuilt image. In that case, download the image from the Releases list and follow the instructions to upload it to docker.

You may also want to uncomment lines at the end of the file to save the database permanently to the host machine.

To start container with Docker Compose, use the command:

```bash
docker-compose up -d
```

To follow logs, use the following command (press <kbd>Ctrl</kbd> + <kbd>C</kbd> to cancel log following)

```bash
docker-compose logs -f -n 10
# or
docker logs netspot_control -f -n 10
```

To stop and remove the container, give the following command:

```bash
docker-compose down
```

## Docker

**Note!** You don't need to follow these instructions if you used Docker Compose above.

Building docker file:

````bash
docker build --tag=netspot_control .
````

Running `netspot_control` with docker:

```bash
docker run --detach --name=netspot_control --cap-add=NET_ADMIN --network=host netspot_control
```

You can now stop container with:

```bash
docker stop netspot_control
```

And start it again with:

```bash
docker start netspot_control
```

Server is configured to listen port 80 by default. You should be able to connect with browser to: http://your-docker-host/

If you are running docker on local machine try opening: http://localhost/

For using another port add `--env=ROCKET_PORT=<port number>` to docker run command. For example:

```bash
docker run --detach --name=netspot_control --cap-add=NET_ADMIN --network=host --env=ROCKET_PORT=3000 netspot_control
```

### Database

Database is stored into the `/var/lib/netspot_control/` path in the container. This can be changed by adding `--env=DB_FILE_PATH=/my/custom/path` to docker arguments. However, that will only change the location inside the container and is not persistent. 

For a persistent database, we have two options: volumes and bind mounts.

See https://docs.docker.com/storage/ for more details.

#### Using volume

Create volume

```bash
docker volume create netspot_control_volume
```

Start container with the volume

```bash
docker run --detach --name=netspot_control --cap-add=NET_ADMIN --network=host \
--mount source=netspot_control_volume,target=/var/lib/netspot_control \
netspot_control
```

#### Bind mount

Start container with bind mount

```bash
docker run --detach --name=netspot_control --cap-add=NET_ADMIN --network=host \
-v /my/local/db/path:/var/lib/netspot_control \
netspot_control
```

### Show netspot messages

By default, netspot statistic messages are not printed to standard output. However, this feature can be enabled with the SHOW_NETSPOT_MESSAGES environment variable. Add the following to the docker command to enable the output: `--env=SHOW_NETSPOT_MESSAGES=1`

We can now see messages from the containers log:

```bash
docker logs netspot_control
```

Adding a `--follow` keeps printing the output from the container. Press Ctrl+C to stop.

```bash
docker logs netspot_control --follow
```

## TODO

- [ ] CORS ?
- [ ] Authorization ?
- [ ] Integration tests
- [ ] Re-generate GitHub actions with sifis-generate

<!-- Links -->
[actions]: https://github.com/sifis-home/wp4-edge_ids/actions
[codecov]: https://codecov.io/gh/sifis-home/wp4-edge_ids
[license]: LICENSE

<!-- Badges -->
[actions badge]: https://github.com/sifis-home/wp4-edge_ids/workflows/netspot_control/badge.svg
[codecov badge]: https://codecov.io/gh/sifis-home/wp4-edge_ids/branch/master/graph/badge.svg
[license badge]: https://img.shields.io/badge/license-MIT-blue.svg
