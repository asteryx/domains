-- Your SQL goes here

CREATE TABLE domain_status (
    id SERIAL PRIMARY KEY,
    date TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    loading_time INTEGER DEFAULT 0 NOT NULL,
    status_code INTEGER DEFAULT 0 NOT NULL,
    headers VARCHAR NOT NULL,
    filename VARCHAR NOT NULL,
    domain_id INTEGER NOT NULL,
    FOREIGN KEY(domain_id) REFERENCES domain(id)
);

CREATE INDEX domain_status_date_index ON domain_status (
	date ASC
);