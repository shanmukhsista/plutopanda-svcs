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