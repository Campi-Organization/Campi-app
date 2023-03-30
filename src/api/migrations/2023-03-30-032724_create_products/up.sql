CREATE TABLE products (
   id CHAR(36) PRIMARY KEY,
   store_id CHAR(36),
   seller_id CHAR(36),
   category_id INT,
   name VARCHAR(255),
   description VARCHAR(255),
   price INT,
   active BOOLEAN DEFAULT false,
   INDEX (id),
   FOREIGN KEY (store_id) REFERENCES stores (id) ON DELETE CASCADE,
   FOREIGN KEY (seller_id) REFERENCES users (id) ON DELETE CASCADE,
   FOREIGN KEY (category_id) REFERENCES categories (id) ON DELETE CASCADE
);