ALTER TABLE public.robots
    ADD COLUMN max_power integer NOT NULL DEFAULT 0;

ALTER TABLE public.robots
    ADD COLUMN recharge_rate integer NOT NULL DEFAULT 0;