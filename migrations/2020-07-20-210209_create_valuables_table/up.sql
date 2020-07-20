-- Your SQL goes here

DROP TABLE public.valuables;

CREATE TABLE public.valuables
(
    id bigint NOT NULL DEFAULT nextval('valuables_id_seq'::regclass),
    q integer NOT NULL,
    r integer NOT NULL,
    kind character varying(64) COLLATE pg_catalog."default" NOT NULL,
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