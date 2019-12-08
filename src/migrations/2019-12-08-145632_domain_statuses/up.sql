-- Your SQL goes here

CREATE TABLE domains_status (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    date VARCHAR DEFAULT CURRENT_TIMESTAMP NOT NULL,
    loading_time INTEGER DEFAULT 0 NOT NULL,
    headers VARCHAR NOT NULL,
    domain_id INTEGER NOT NULL,
    FOREIGN KEY(domain_id) REFERENCES domains(id)
);

CREATE UNIQUE INDEX domains_status_date_unique_index ON domains_status (
	date ASC
);