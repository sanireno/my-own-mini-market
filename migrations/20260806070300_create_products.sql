CREATE TABLE products (
                          id BIGSERIAL PRIMARY KEY,
                          name TEXT NOT NULL,
                          description TEXT NOT NULL,
                          price BIGINT NOT NULL CHECK (price >= 0),
                          stock INTEGER NOT NULL CHECK (stock >= 0)
);
