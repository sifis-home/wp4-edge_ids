# WP4 netspot

This repository is for implementing SIFIS-Home WP4 Multi-Level Analysis using the SPOT algorithm. The solution is planned to run as a docker container where the controlling program provides HTTP REST API for other services. The controlling service is designed to run netspot programs and collect data to the cache database.

Below is initial plans for the service.

![Initial plans for the solution](docs/NetSpotControl.png)

## Project folders

* docs – Documentation and plans
* netspot_control - Rust project for developing the planned service
  * Currently implements only listing of host system network interface

## Netspot

We are planning to use the following Go Implementation of SPOT
https://github.com/asiffer/netspot

Python version of the algorithm is also available:
https://github.com/Amossys-team/SPOT

Project Page:
https://asiffer.github.io/netspot/

## Docker

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

## Development

For easier development on the local machine, we recommend you install the netspot on your systems to make it available for the netspot_control application. When you run the netspot_control with `cargo run`, it will use port 8000 by default. You can change the port by using the `ROCKET_PORT` environment variable.

## TODO

- [x] OpenAPI specification for the service
  - [x] Version 0.1.1 done — [yaml](docs/netspot-control-api.yaml) | [json](netspot_control/static/design/openapi.json)
  - [x] Available from the `netspot_control` service itself
- [x] Checking for required technologies
- [x] Dockerfile
- [ ] Writing `netspot_control` application:
  - [x] Design of program parts and interactions
  - [ ] Structures
    - [x] Netspot configuration
      - [x] Making TOML configuration for netspot
    - [x] Status
    - [x] Statuses
    - [ ] Statistics
    - [ ] Webhook
    - [ ] Webhooks
  - [ ] Database handler
    - [x] Configuration endpoints
    - [ ] Statistics
    - [ ] Webhooks
  - [ ] NetspotProcess (WIP)
  - [ ] NetspotManager (WIP)
    - [ ] SocketListener
    - [ ] NetspotSocket
    - [ ] Status endpoints (WIP)
  - [ ] Webhooks
    - [ ] Webhook endpoints
  - [x] Using `rocket_okapi` could provide OpenAPI generated from the implementation
  - [ ] CORS ?
  - [ ] Authorization ?
- [ ] WP2 Checklist
  - [ ] GitHub Action
    - [ ] Code coverage
    - [ ] Continous delivery
- [ ] Integration tests
- [ ] Example scripts
