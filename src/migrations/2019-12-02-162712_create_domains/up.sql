-- Your SQL goes here

CREATE TABLE domains (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    url VARCHAR NOT NULL,
    status INTEGER DEFAULT 1 NOT NULL,
    author INTEGER NOT NULL,
    FOREIGN KEY(author) REFERENCES users(id)
);

CREATE UNIQUE INDEX domains_url_unique_index ON domains (
	url	ASC
);
