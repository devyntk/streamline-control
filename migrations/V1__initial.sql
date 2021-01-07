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

CREATE TABLE user_group
(
    id   integer PRIMARY KEY,
    name text
);

CREATE TABLE group_users
(
    user_id  integer,
    group_id integer,
    FOREIGN KEY (user_id) REFERENCES user (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (group_id) REFERENCES  user_group(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE group_permissions
(
    group_id integer,
    perm_id  integer,
    FOREIGN KEY (group_id) REFERENCES  user_group(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE event
(
    id    integer PRIMARY KEY,
    name  text,
    start datetime,
    end   datetime
);

CREATE TABLE user_events
(
    user_id  integer,
    event_id integer,
    FOREIGN KEY (user_id) REFERENCES user (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (event_id) REFERENCES event (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE session
(
    id       integer PRIMARY KEY,
    user_id  integer,
    event_id integer,
    token    text,
    csrf     text,
    FOREIGN KEY (user_id) REFERENCES user (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE ,
    FOREIGN KEY (event_id) REFERENCES event (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE song
(
    uri              text PRIMARY KEY,
    name             text,
    length_ms        integer,
    album_name       text,
    artist_name      text,
    acousticness     real,
    danceability     real,
    energy           real,
    instrumentalness real,
    liveness         real,
    loudness         real,
    speechiness      real,
    valence          real,
    tempo            real,
    popularity       integer
);

CREATE TABLE playlist
(
    id     integer PRIMARY KEY,
    name   text,
    system integer
);

CREATE TABLE playlist_entry
(
    song_uri text,
    added_by integer,
    playlist integer,
    FOREIGN KEY (song_uri) REFERENCES song(uri),
    FOREIGN KEY (added_by) REFERENCES user(id),
    FOREIGN KEY (playlist) REFERENCES playlist(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE match
(
    id              text,
    division        integer,
    event           integer,
    intro_song      text,
    warmup_song     text,
    match_song      text,
    match_ms_offset integer,
    finish_song     text,
    post_song       text,
    FOREIGN KEY (event) REFERENCES event(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (intro_song) REFERENCES song(uri),
    FOREIGN KEY (warmup_song) REFERENCES song(uri),
    FOREIGN KEY (match_song) REFERENCES song(uri),
    FOREIGN KEY (finish_song) REFERENCES song(uri),
    FOREIGN KEY (post_song) REFERENCES song(uri)
);

