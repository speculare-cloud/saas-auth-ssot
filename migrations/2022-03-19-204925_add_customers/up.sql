CREATE TABLE customers (
	id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
	email VARCHAR(255) UNIQUE NOT NULL
);

CREATE INDEX customers_id ON customers(id);
CREATE INDEX customers_email ON customers(email);