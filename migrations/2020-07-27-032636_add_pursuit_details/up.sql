ALTER TABLE public.robots
    ADD COLUMN pursuit_id bigint NOT NULL default -1;

ALTER TABLE public.robots
    ADD COLUMN pursuit_last_q INTEGER NOT NULL default -1;

ALTER TABLE public.robots
    ADD COLUMN pursuit_last_r INTEGER NOT NULL default -1;