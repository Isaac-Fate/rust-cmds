-- Add migration script here

CREATE TABLE IF NOT EXISTS dirs (
    id              INTEGER  PRIMARY KEY AUTOINCREMENT,
    path            text NOT NULL,
    last_visit_time text NOT NULL
)
