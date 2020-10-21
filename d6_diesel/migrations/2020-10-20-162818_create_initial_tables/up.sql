-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  password VARCHAR NOT NULL
);

CREATE TABLE polls(
  id SERIAL PRIMARY KEY,
  question VARCHAR NOT NULL,
  options VARCHAR NOT NULL
);

CREATE TABLE responses(
  poll_id INTEGER REFERENCES polls(id),
  user_id INTEGER REFERENCES users(id),
  selected INTEGER,
  PRIMARY KEY (poll_id, user_id)
);