-- Create Subscriptions Table
CREATE TABLE users(
id uuid NOT NULL,
PRIMARY KEY (id),
email TEXT NOT NULL UNIQUE,
password TEXT NOT NULL,
username TEXT NOT NULL UNIQUE,
created_at TEXT NOT NULL
);