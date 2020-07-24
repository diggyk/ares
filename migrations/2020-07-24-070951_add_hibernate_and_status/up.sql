-- Your SQL goes here

ALTER TABLE public.robots
    ADD COLUMN hibernate_countdown integer NOT NULL DEFAULT -1;

ALTER TABLE public.robots
    ADD COLUMN status_text character varying(64) NOT NULL DEFAULT '';