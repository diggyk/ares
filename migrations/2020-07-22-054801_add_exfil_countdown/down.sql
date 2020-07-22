-- This file should undo anything in `up.sql`

ALTER TABLE public.robots
    DROP COLUMN exfil_countdown;