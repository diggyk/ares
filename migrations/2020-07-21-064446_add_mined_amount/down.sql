-- This file should undo anything in `up.sql`

ALTER TABLE public.robots
    DROP COLUMN mined_amount;