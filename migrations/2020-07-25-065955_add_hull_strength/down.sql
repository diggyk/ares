-- This file should undo anything in `up.sql`

ALTER TABLE public.robots
    DROP COLUMN hull_strength;

ALTER TABLE public.robots
    DROP COLUMN max_hull_strength;