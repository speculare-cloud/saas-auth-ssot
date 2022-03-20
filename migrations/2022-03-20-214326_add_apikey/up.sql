CREATE TABLE apikeys (
	id BIGSERIAL PRIMARY KEY NOT NULL,
	key TEXT NOT NULL,
	host_uuid TEXT,
	customer_id uuid NOT NULL,
	berta TEXT NOT NULL
);

CREATE INDEX apikeys_key ON apikeys(key);
CREATE INDEX apikeys_host_uuid ON apikeys(host_uuid);