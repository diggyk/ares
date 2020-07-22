-- Your SQL goes here

ALTER TABLE public.robots
    ADD COLUMN exfil_countdown integer NOT NULL DEFAULT -1;