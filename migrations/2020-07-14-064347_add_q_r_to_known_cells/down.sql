-- This file should undo anything in `up.sql`

ALTER TABLE public.robot_known_cells
    DROP COLUMN q;

ALTER TABLE public.robot_known_cells
    DROP COLUMN r;