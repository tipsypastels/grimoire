CREATE TABLE nodes (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  path TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  kind TEXT NOT NULL,
  text TEXT NOT NULL
);

CREATE INDEX nodes_idx_name ON nodes (name);
CREATE INDEX nodes_idx_kind ON nodes (kind);

CREATE TABLE node_tags (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  node_id INTEGER NOT NULL REFERENCES nodes (id) ON DELETE CASCADE,
  tag TEXT,
  UNIQUE (node_id, tag)
);

CREATE INDEX node_tags_idx_tag ON node_tags (tag);

CREATE TABLE node_dependencies (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  from_id INTEGER NOT NULL REFERENCES nodes (id) ON DELETE CASCADE,
  to_id INTEGER NOT NULL REFERENCES nodes (id) ON DELETE CASCADE,
  UNIQUE (from_id, to_id)
);
