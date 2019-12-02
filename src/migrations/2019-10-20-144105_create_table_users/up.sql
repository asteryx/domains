-- Your SQL goes here

CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    name VARCHAR DEFAULT '' NOT NULL
);

CREATE UNIQUE INDEX users_email_unique_index ON users (
	email	ASC
);
