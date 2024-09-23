-- Add migration script here
BEGIN;
    -- Backfill `status` for old entries
    UPDATE subscriptions
        SET status = 'confirmed'
        WHERE status IS NULL;
    -- make status required
    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;
