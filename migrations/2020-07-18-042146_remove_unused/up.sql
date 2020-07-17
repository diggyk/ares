-- Your SQL goes here

ALTER TABLE public.robots
    DROP COLUMN gridcell;

ALTER TABLE public.robots
    DROP COLUMN components;

ALTER TABLE public.robots
    DROP COLUMN configs;