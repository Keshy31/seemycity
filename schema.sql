-- Enable PostGIS extension if not already enabled (Fly images usually have it, but this ensures it)
CREATE EXTENSION IF NOT EXISTS postgis;

-- Table to store municipality details 
CREATE TABLE municipalities (
	id varchar NOT NULL,          -- Corresponds to municipal_geometries.munic_id
	"name" text NOT NULL,
	province text NOT NULL,
	-- geojson column removed as requested
	population real NULL, -- Changed from int4 to real
	classification text NULL,
	address text NULL,
	website text NULL,
	phone text NULL,
	district_id varchar NULL,
	district_name text NULL,
	CONSTRAINT municipalities_pkey PRIMARY KEY (id)
);

-- Table to store municipality geometry
CREATE TABLE municipal_geometries (
	ogc_fid serial4 NOT NULL,
	geom public.geometry(geometry, 4326) NULL,
	munic_id varchar NOT NULL, -- Renamed from cat_b and set to NOT NULL
	CONSTRAINT municipal_geometries_pkey PRIMARY KEY (ogc_fid),
    CONSTRAINT municipal_geometries_municipalities_fk FOREIGN KEY (munic_id) REFERENCES public.municipalities(id) ON DELETE CASCADE ON UPDATE CASCADE -- Updated FK column name
);
CREATE INDEX municipal_geometries_geom_geom_idx ON public.municipal_geometries USING gist (geom);
CREATE INDEX municipal_geometries_munic_id_idx ON public.municipal_geometries USING btree (munic_id); -- Updated index column name and index name

-- Table to store cached financial data and scores
CREATE TABLE financial_data (
	id uuid NOT NULL DEFAULT gen_random_uuid(), -- Use default UUID generation
	municipality_id varchar NULL,
	"year" int4 NOT NULL,
	revenue numeric NULL,
	expenditure numeric NULL,
	capital_expenditure numeric NULL,
	debt numeric NULL,
	audit_outcome text NULL,
	score numeric NULL,
	created_at timestamptz DEFAULT now() NULL,
	CONSTRAINT financial_data_pkey PRIMARY KEY (id),
    CONSTRAINT financial_data_municipalities_fk FOREIGN KEY (municipality_id) REFERENCES public.municipalities(id) ON DELETE SET NULL ON UPDATE CASCADE -- Consider ON DELETE behavior
);
CREATE INDEX financial_data_municipality_id_year_idx ON public.financial_data USING btree (municipality_id, year);

-- Optional: Add comments for clarity (good practice)
COMMENT ON TABLE municipalities IS 'Stores static details for South African municipalities.';
COMMENT ON TABLE municipal_geometries IS 'Stores geographic boundaries for South African municipalities.';
COMMENT ON TABLE financial_data IS 'Stores cached financial metrics and calculated scores for municipalities, linked by municipality_id and year.';
