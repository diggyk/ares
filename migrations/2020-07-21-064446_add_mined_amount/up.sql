-- Your SQL goes here

ALTER TABLE public.robots
    ADD COLUMN mined_amount integer NOT NULL DEFAULT 0;