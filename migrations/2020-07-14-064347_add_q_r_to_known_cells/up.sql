-- Your SQL goes here

ALTER TABLE public.robot_known_cells
    ADD COLUMN q integer NOT NULL;

ALTER TABLE public.robot_known_cells
    ADD COLUMN r integer NOT NULL;