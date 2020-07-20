-- This file should undo anything in `up.sql`

ALTER TABLE public.robots
    DROP COLUMN max_power;

ALTER TABLE public.robots
    DROP COLUMN recharge_rate;