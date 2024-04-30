#!/usr/bin/bash

PGPASSWORD=postgres psql -h localhost --port 5432 -d craby_city --username postgres -a -f tables.sql
PGPASSWORD=postgres psql -h localhost --port 5432 -d craby_city --username postgres -a -f queries.sql
