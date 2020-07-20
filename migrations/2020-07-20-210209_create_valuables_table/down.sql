-- This file should undo anything in `up.sql`

DROP TABLE public.valuables;

CREATE TABLE public.valuables
(
    id bigint NOT NULL DEFAULT nextval('"valuables_id_seq"'::regclass),
    q integer NOT NULL,
    r integer NOT NULL,
    gridcell integer NOT NULL,
    type smallint NOT NULL,
    amount integer NOT NULL,
    CONSTRAINT valuables_pkey PRIMARY KEY (id)
)
