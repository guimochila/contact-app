-- Add migration script here
CREATE TABLE IF NOT EXISTS contacts (
  id TEXT PRIMARY KEY NOT NULL,
  first TEXT NOT NULL,
  last TEXT,
  email TEXT NOT NULL,
  phone TEXT
);
