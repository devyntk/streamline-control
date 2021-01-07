CREATE TABLE user (
                      id integer PRIMARY KEY AUTOINCREMENT,
                      name text,
                      created datetime,
                      pw text
);

CREATE TABLE user_group (
                       id integer PRIMARY KEY AUTOINCREMENT,
                       name text
);

CREATE TABLE group_users (
                             user_id integer,
                             group_id integer
);

CREATE TABLE group_permissions (
                                   group_id integer,
                                   perm_id integer
);

CREATE TABLE event (
                       id integer PRIMARY KEY AUTOINCREMENT,
                       name text,
                       start datetime,
                       end datetime
);

CREATE TABLE user_events (
                             user_id integer,
                             event_id integer
);

CREATE TABLE session (
                         id integer PRIMARY KEY AUTOINCREMENT,
                         user_id integer,
                         event_id integer,
                         token text,
                         csrf text
);

CREATE TABLE song (
                      uri text,
                      name text,
                      length integer,
                      acousticness text,
                      danceability text,
                      energy text,
                      instrumentalness text,
                      liveness text,
                      loudness text,
                      speechiness text,
                      valence text,
                      tempo text
);

CREATE TABLE playlist_entry (
                                song_uri text,
                                added_by integer,
                                playlist text
);

CREATE TABLE match (
                       id text,
                       division integer,
                       event integer,
                       song text
);

