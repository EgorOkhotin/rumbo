-- Your SQL goes here
CREATE TABLE jobs (
    name TEXT PRIMARY KEY,
    last_invocation TIMESTAMP NOT NULL,
    sleep_time INTERVAL NOT NULL 
)