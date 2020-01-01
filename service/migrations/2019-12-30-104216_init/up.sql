-- Your SQL goes here
CREATE TABLE `jumps` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `from` VARCHAR NOT NULL,
  `to` VARCHAR NOT NULL,
  `create_time` DATETIME NOT NULL
);
CREATE INDEX `from_on_jumps_index` on `jumps` (`from`);
CREATE TABLE `stats` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `ip` VARCHAR NOT NULL,
  `jump_from` VARCHAR NOT NULL,
  `url` VARCHAR NOT NULL,
  `time` DATETIME NOT NULL
);
CREATE INDEX `short_link_on_stats_index` on `stats` (`jump_from`);
CREATE INDEX `ip_on_stats_index` on `stats` (`ip`);