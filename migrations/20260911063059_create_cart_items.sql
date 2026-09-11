-- Add migration script here
CREATE TABLE cart_items (
                            id BIGSERIAL PRIMARY KEY,

                            user_id BIGINT NOT NULL REFERENCES users(id),
                            product_id BIGINT NOT NULL REFERENCES products(id),

                            quantity INTEGER NOT NULL CHECK (quantity > 0),

                            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);