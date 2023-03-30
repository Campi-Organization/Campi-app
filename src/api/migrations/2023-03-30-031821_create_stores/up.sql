CREATE TABLE stores (
   id CHAR(36) PRIMARY KEY,
   owner_id CHAR(36),
   description VARCHAR(255),
   name VARCHAR(255),
   status BOOLEAN DEFAULT false,
   location VARCHAR(255),
   active BOOLEAN DEFAULT false,
   INDEX (id),
   FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE
);