CREATE TABLE users (
  id BIGSERIAL PRIMARY KEY,
  uid Uuid NOT NULL,
  username VARCHAR NOT NULL Unique,
  email VARCHAR NOT NULL Unique,
  password VARCHAR  NOT NULL,
  sign_in_count INTEGER NOT NULL DEFAULT 0,
  current_sign_in_at TIMESTAMP,
  last_sign_in_at TIMESTAMP,
  deleted_at TIMESTAMP ,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)