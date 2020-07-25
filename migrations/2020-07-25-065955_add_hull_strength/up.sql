-- Your SQL goes here

ALTER TABLE public.robots
    ADD COLUMN hull_strength integer NOT NULL DEFAULT -1;

ALTER TABLE public.robots
    ADD COLUMN max_hull_strength integer NOT NULL DEFAULT -1;