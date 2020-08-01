ALTER TABLE public.robots
    ADD COLUMN attacked BIGINT NOT NULL default -1;

ALTER TABLE public.robots
    ADD COLUMN damage_done INTEGER NOT NULL default -1;