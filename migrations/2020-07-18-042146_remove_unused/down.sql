-- This file should undo anything in `up.sql`

ALTER TABLE public.robots
    ADD COLUMN gridcell integer;

ALTER TABLE public.robots
    ADD COLUMN components json;

ALTER TABLE public.robots
    ADD COLUMN configs json;