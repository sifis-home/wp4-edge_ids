-- Changing time to BIGINT for alarms
CREATE TABLE alarms_new
(
    id INTEGER NOT NULL PRIMARY KEY,
    time BIGINT NOT NULL,
    message TEXT NOT NULL
);
INSERT INTO alarms_new(time, message) SELECT time, message FROM alarms;
DROP TABLE alarms;
ALTER TABLE alarms_new RENAME TO alarms;

-- Changing time to BIGINT for data
CREATE TABLE data_new
(
    id INTEGER NOT NULL PRIMARY KEY,
    time BIGINT NOT NULL,
    message TEXT NOT NULL
);
INSERT INTO data_new(time, message) SELECT time, message FROM data;
DROP TABLE data;
ALTER TABLE data_new RENAME TO data;
