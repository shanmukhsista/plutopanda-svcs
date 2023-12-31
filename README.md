

# Database Setup

```
echo DATABASE_URL=postgres://ppuser:ppuser@localhost:5435/ppsvcsdb > .env
```

```angular2html
brew install libpq
export DYLD_LIBRARY_PATH=/usr/local/opt/libpq/lib:/usr/local/opt/libiconv/lib:$DYLD_LIBRARY_PATH

cargo install sqlx-cli --no-default-features --features native-tls,postgres

```

```
-- Your SQL goes here
create table projects
(
    id    bigserial PRIMARY KEY,
    title VARCHAR NOT NULL,
    is_complete boolean default false,
    created_on  timestamp not null default current_timestamp,
    updated_on timestamp
);

CREATE TABLE work_items
(
    id           SERIAL PRIMARY KEY,
    title        VARCHAR   NOT NULL,
    summary      TEXT      NOT NULL,
    created_on   timestamp not null default current_timestamp,
    started_on   timestamp null,
    completed_on timestamp null,
    duration_ms  integer ,
    project_id   bigserial references projects(id)
);


-- This file should undo anything in `up.sql`
drop table if exists work_items ;

drop table if exists projects ;

sqlx migrate add update_timetsamp_fields

```