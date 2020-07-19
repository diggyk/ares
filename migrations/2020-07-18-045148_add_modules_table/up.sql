-- Your SQL goes here

CREATE TABLE public.robot_modules
(
    robot_id bigint NOT NULL,
    m_collector character varying(64) NOT NULL DEFAULT 'basic',
    m_drivesystem character varying(64) NOT NULL DEFAULT 'basic',
    m_exfilbeacon character varying(64) NOT NULL DEFAULT 'basic',
    m_hull character varying(64) NOT NULL DEFAULT 'basic',
    m_memory character varying(64) NOT NULL DEFAULT 'basic',
    m_power character varying(64) NOT NULL DEFAULT 'basic',
    m_scanner character varying(64) NOT NULL DEFAULT 'basic',
    m_weapons character varying(64) NOT NULL DEFAULT 'basic',
    PRIMARY KEY (robot_id),
    CONSTRAINT robot_id_fk FOREIGN KEY (robot_id)
        REFERENCES public.robots (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE public.robot_modules
    OWNER to plexms;

COMMENT ON TABLE public.robot_modules
    IS 'Holds the data about which modules, if any, are loaded for a robot';