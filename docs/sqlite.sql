-- This script creates tables and default configuration

CREATE TABLE configurations
(
    id INTEGER NOT NULL PRIMARY KEY,
    config JSON NOT NULL
);

CREATE TABLE alarms
(
    id INTEGER NOT NULL PRIMARY KEY,
    time INTEGER NOT NULL,
    message JSON NOT NULL
);

CREATE TABLE data
(
    id INTEGER NOT NULL PRIMARY KEY,
    time INTEGER NOT NULL,
    message JSON NOT NULL
);

CREATE TABLE webhooks
(
    id INTEGER NOT NULL PRIMARY KEY,
    config JSON NOT NULL
);

INSERT INTO configurations(config) VALUES(
    '{"configuration":{"name":"Default configuration","device":"any","promiscuous":true},"spot":{"depth":50,"q":0.00001,"n_init":2000,"level":0.98,"up":true,"down":false,"alert":true,"bounded":true,"max_excess":200},"stats":{"avg_pkg_size":{"enabled":true,"max_excess":1},"perf":{"enabled":true,"up":false},"r_arp":{"enabled":true},"r_syn":{"enabled":true},"traffic":{"enabled":true}}}'
);

