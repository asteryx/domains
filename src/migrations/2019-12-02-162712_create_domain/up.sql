-- Your SQL goes here

CREATE TABLE domain (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    url VARCHAR NOT NULL,
    state INTEGER DEFAULT 1 NOT NULL,
    author INTEGER NOT NULL,
    FOREIGN KEY(author) REFERENCES users(id)
);

CREATE UNIQUE INDEX domain_url_unique_index ON domain (
	url	ASC
);
