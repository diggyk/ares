-- This file should undo anything in `up.sql`

ALTER TABLE public.robots
    DROP COLUMN hibernate_countdown;

ALTER TABLE public.robots
    DROP COLUMN status_text;