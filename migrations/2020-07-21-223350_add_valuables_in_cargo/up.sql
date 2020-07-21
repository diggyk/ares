-- Your SQL goes here

ALTER TABLE public.robots
    ADD COLUMN val_inventory integer NOT NULL DEFAULT 0;