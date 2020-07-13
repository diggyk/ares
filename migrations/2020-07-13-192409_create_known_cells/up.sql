CREATE TABLE public.robot_known_cells
(
    robot_id bigint NOT NULL,
    gridcell_id integer NOT NULL,
    discovery_time timestamp without time zone NOT NULL,
    PRIMARY KEY (robot_id),
    CONSTRAINT gridcell_id_key FOREIGN KEY (gridcell_id)
        REFERENCES public.gridcells (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID,
    CONSTRAINT robot_id_key FOREIGN KEY (robot_id)
        REFERENCES public.robots (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE public.robot_known_cells
    OWNER to plexms;

COMMENT ON TABLE public.robot_known_cells
    IS 'The list of known cells by a robot';