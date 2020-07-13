-- Your SQL goes here
-- Table: public.gridcells

-- DROP TABLE public.gridcells;

CREATE TABLE public.gridcells
(
    id integer NOT NULL DEFAULT 0,
    q integer NOT NULL,
    r integer NOT NULL,
    edge0 smallint NOT NULL DEFAULT 1,
    edge60 smallint NOT NULL DEFAULT 1,
    edge120 smallint NOT NULL DEFAULT 1,
    edge180 smallint NOT NULL DEFAULT 1,
    edge240 smallint NOT NULL DEFAULT 1,
    edge300 smallint NOT NULL DEFAULT 1,
    CONSTRAINT gridcells_pkey PRIMARY KEY (id),
    CONSTRAINT loc UNIQUE (q, r)
)

TABLESPACE pg_default;

ALTER TABLE public.gridcells
    OWNER to plexms;

GRANT ALL ON TABLE public.gridcells TO ares;

GRANT TRIGGER, SELECT, REFERENCES ON TABLE public.gridcells TO ares_api;

GRANT ALL ON TABLE public.gridcells TO plexms;

COMMENT ON TABLE public.gridcells
    IS 'Grid cell information';

COMMENT ON CONSTRAINT loc ON public.gridcells
    IS 'Unique location';
-- Index: loc_index

-- DROP INDEX public.loc_index;

CREATE SEQUENCE public.robot_id_seq
    CYCLE
    INCREMENT 1
    START 1000
    MINVALUE 1000
    MAXVALUE 9000000;

ALTER SEQUENCE public.robot_id_seq
    OWNER TO plexms;

CREATE INDEX loc_index
    ON public.gridcells USING btree
    (q ASC NULLS LAST, r ASC NULLS LAST)
    TABLESPACE pg_default;

COMMENT ON INDEX public.loc_index
    IS 'Index locations';

-- Table: public.robots

-- DROP TABLE public.robots;

CREATE TABLE public.robots
(
    id bigint NOT NULL DEFAULT nextval('robot_id_seq'::regclass),
    name character varying(16) COLLATE pg_catalog."default" NOT NULL,
    owner integer,
    affiliation integer,
    q integer NOT NULL,
    r integer NOT NULL,
    orientation smallint NOT NULL,
    gridcell integer,
    components json,
    configs json,
    CONSTRAINT robot_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE public.robots
    OWNER to plexms;

GRANT ALL ON TABLE public.robots TO ares;

GRANT TRIGGER, SELECT, REFERENCES ON TABLE public.robots TO ares_api;

GRANT ALL ON TABLE public.robots TO plexms;

COMMENT ON TABLE public.robots
    IS 'Robots in the field';

-- Table: public.valuables

-- DROP TABLE public.valuables;

CREATE SEQUENCE public.valuables_id_seq
    CYCLE
    INCREMENT 1
    START 1000
    MINVALUE 1000
    MAXVALUE 9000000;

ALTER SEQUENCE public.valuables_id_seq
    OWNER TO plexms;

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

TABLESPACE pg_default;

ALTER TABLE public.valuables
    OWNER to plexms;

GRANT ALL ON TABLE public.valuables TO ares;

GRANT TRIGGER, SELECT, REFERENCES ON TABLE public.valuables TO ares_api;

GRANT ALL ON TABLE public.valuables TO plexms;

COMMENT ON TABLE public.valuables
    IS 'Location of valuables';