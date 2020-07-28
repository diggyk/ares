ALTER TABLE public.robots
    ADD COLUMN attacked_from INTEGER NOT NULL default -1;

ALTER TABLE public.robots
    ADD COLUMN attacked_by BIGINT NOT NULL default -1;