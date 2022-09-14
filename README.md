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

## Roadmap

- [x] OpenAPI specification for the service
  - [x] Version 0.1.0 done — [html](docs/netspot-control-api.html) | [yaml](docs/netspot-control-api.yaml)
- [ ] Checking for required technologies – The plan is to use Rust if possible
  - [x] Listing available network interfaces
  - [ ] Unix sockets research
  - [ ] Controlling netspot processes
  - [ ] Cache database – Sqlite planned
- [ ] Writing implementation to match the designed OpenAPI specification
  - [ ] Using `rocket_okapi` could provide OpenAPI generated from the implementation
- [ ] CORS ?
- [ ] Authorization ?
- [ ] Docker container
