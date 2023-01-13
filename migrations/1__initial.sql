PRAGMA foreign_keys = ON;

CREATE TABLE user
(
    id              integer PRIMARY KEY,
    username        text,
    display_name    text,
    email           text,
    created         datetime,
    pw              text
);

CREATE TABLE session
(
    id       integer PRIMARY KEY,
    user_id  integer,
    token    text,
    csrf     text,
    last_ip  text,
    ip_time  datetime,
    FOREIGN KEY (user_id) REFERENCES user (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

