-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(250) NOT NULL,
    password VARCHAR(125) NOT NULL,
    name VARCHAR(250) DEFAULT '' NOT NULL
);

CREATE UNIQUE INDEX users_email_unique_index ON users (
	email	ASC
);
