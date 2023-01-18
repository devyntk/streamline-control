PRAGMA foreign_keys = ON;

CREATE TABLE user
(
    id           integer PRIMARY KEY,
    username     text,
    display_name text,
    email        text,
    created      datetime,
    pw           text
);

CREATE TABLE config
(
    key   text PRIMARY KEY,
    value text
);

