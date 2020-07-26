-- Table: public.robots

DROP TABLE public.robot_known_cells;
DROP TABLE public.robot_modules;
DROP TABLE public.robots;
DROP SEQUENCE public.robot_id_seq;

-- 

CREATE SEQUENCE public.robot_id_seq
    CYCLE
    INCREMENT 1
    START 1000
    MINVALUE 1000
    MAXVALUE 9000000
    CACHE 1;

ALTER SEQUENCE public.robot_id_seq
    OWNER TO plexms;

-- 

CREATE TABLE public.robots
(
    id bigint NOT NULL DEFAULT nextval('robot_id_seq'::regclass),
    name character varying(16) COLLATE pg_catalog."default" NOT NULL,
    owner integer,
    affiliation integer,
    q integer NOT NULL,
    r integer NOT NULL,
    orientation smallint NOT NULL,
    power integer NOT NULL DEFAULT 0,
    max_power integer NOT NULL DEFAULT 0,
    recharge_rate integer NOT NULL DEFAULT 0,
    hull_strength integer NOT NULL DEFAULT '-1'::integer,
    max_hull_strength integer NOT NULL DEFAULT '-1'::integer,
    mined_amount integer NOT NULL DEFAULT 0,
    val_inventory integer NOT NULL DEFAULT 0,
    max_val_inventory integer NOT NULL DEFAULT 0,
    exfil_countdown integer NOT NULL DEFAULT '-1'::integer,
    hibernate_countdown integer NOT NULL DEFAULT '-1'::integer,
    status_text character varying(64) COLLATE pg_catalog."default" NOT NULL DEFAULT ''::character varying,
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

-- 
CREATE TABLE public.robot_known_cells
(
    robot_id bigint NOT NULL,
    gridcell_id integer NOT NULL,
    discovery_time timestamp without time zone NOT NULL,
    q integer NOT NULL,
    r integer NOT NULL,
    CONSTRAINT robot_known_cells_pkey PRIMARY KEY (robot_id, gridcell_id),
    CONSTRAINT gridcell_id_key FOREIGN KEY (gridcell_id)
        REFERENCES public.gridcells (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT robot_id_key FOREIGN KEY (robot_id)
        REFERENCES public.robots (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE public.robot_known_cells
    OWNER to plexms;

GRANT ALL ON TABLE public.robot_known_cells TO ares_api;

GRANT ALL ON TABLE public.robot_known_cells TO plexms;

COMMENT ON TABLE public.robot_known_cells
    IS 'The list of known cells by a robot';

-- 

CREATE TABLE public.robot_modules
(
    robot_id bigint NOT NULL,
    m_collector character varying(64) COLLATE pg_catalog."default" NOT NULL DEFAULT 'basic'::character varying,
    m_drivesystem character varying(64) COLLATE pg_catalog."default" NOT NULL DEFAULT 'basic'::character varying,
    m_exfilbeacon character varying(64) COLLATE pg_catalog."default" NOT NULL DEFAULT 'basic'::character varying,
    m_hull character varying(64) COLLATE pg_catalog."default" NOT NULL DEFAULT 'basic'::character varying,
    m_memory character varying(64) COLLATE pg_catalog."default" NOT NULL DEFAULT 'basic'::character varying,
    m_power character varying(64) COLLATE pg_catalog."default" NOT NULL DEFAULT 'basic'::character varying,
    m_scanner character varying(64) COLLATE pg_catalog."default" NOT NULL DEFAULT 'basic'::character varying,
    m_weapons character varying(64) COLLATE pg_catalog."default" NOT NULL DEFAULT 'basic'::character varying,
    CONSTRAINT robot_modules_pkey PRIMARY KEY (robot_id),
    CONSTRAINT robot_id_fk FOREIGN KEY (robot_id)
        REFERENCES public.robots (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE public.robot_modules
    OWNER to plexms;

GRANT SELECT ON TABLE public.robot_modules TO ares_api;

GRANT ALL ON TABLE public.robot_modules TO plexms;

COMMENT ON TABLE public.robot_modules
    IS 'Holds the data about which modules, if any, are loaded for a robot';