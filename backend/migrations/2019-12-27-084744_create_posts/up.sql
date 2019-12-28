-- Your SQL goes here
CREATE TABLE links (
  id INTEGER NOT NULL PRIMARY KEY,
  short_link VARCHAR NOT NULL,
  full_link VARCHAR NOT NULL,
  create_time DATETIME NOT NULL
);

CREATE INDEX short_link_on_links_index on links (short_link);

CREATE TABLE stats ( 
  id INTEGER NOT NULL PRIMARY KEY,
  ip VARCHAR NOT NULL,
  short_link VARCHAR NOT NULL,
  access_url VARCHAR NOT NULL
);

CREATE INDEX short_link_on_stats_index on stats (short_link);
CREATE INDEX ip_on_stats_index on stats (ip);