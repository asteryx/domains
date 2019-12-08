-- This file should undo anything in `up.sql`

DROP INDEX domains_status_date_unique_index;
DROP TABLE domains_status;