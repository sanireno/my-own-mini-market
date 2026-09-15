WITH duplicate_totals AS (
    SELECT MIN(id) AS kept_id,
           LEAST(SUM(quantity), 2147483647)::INTEGER AS total_quantity
    FROM cart_items
    GROUP BY user_id, product_id
    HAVING COUNT(*) > 1
)
UPDATE cart_items AS item
SET quantity = duplicate_totals.total_quantity,
    updated_at = NOW()
FROM duplicate_totals
WHERE item.id = duplicate_totals.kept_id;

DELETE FROM cart_items AS newer
USING cart_items AS older
WHERE newer.user_id = older.user_id
  AND newer.product_id = older.product_id
  AND newer.id > older.id;

ALTER TABLE cart_items
    ADD CONSTRAINT cart_items_user_product_unique UNIQUE (user_id, product_id),
    DROP CONSTRAINT cart_items_user_id_fkey,
    ADD CONSTRAINT cart_items_user_id_fkey
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    DROP CONSTRAINT cart_items_product_id_fkey,
    ADD CONSTRAINT cart_items_product_id_fkey
        FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE;
