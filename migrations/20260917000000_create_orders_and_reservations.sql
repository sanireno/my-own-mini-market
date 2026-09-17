ALTER TABLE products
    ADD COLUMN reserved_stock INTEGER NOT NULL DEFAULT 0,
    ADD CONSTRAINT products_reserved_stock_non_negative CHECK (reserved_stock >= 0),
    ADD CONSTRAINT products_reserved_stock_not_above_stock CHECK (reserved_stock <= stock);

CREATE TABLE orders (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id),
    status TEXT NOT NULL DEFAULT 'pending_payment'
        CHECK (status IN ('pending_payment', 'paid', 'cancelled', 'expired')),
    idempotency_key TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    paid_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, idempotency_key)
);

CREATE TABLE order_items (
    id BIGSERIAL PRIMARY KEY,
    order_id BIGINT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id BIGINT NOT NULL REFERENCES products(id),
    product_name TEXT NOT NULL,
    unit_price BIGINT NOT NULL CHECK (unit_price >= 0),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    UNIQUE (order_id, product_id)
);

CREATE INDEX orders_user_id_created_at_idx ON orders (user_id, created_at DESC);
CREATE INDEX orders_pending_expiration_idx ON orders (expires_at)
    WHERE status = 'pending_payment';
CREATE INDEX order_items_order_id_idx ON order_items (order_id);
