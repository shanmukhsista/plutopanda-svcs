-- Add migration script here
alter table work_items alter column started_on type timestamptz ;
-- Add migration script here
alter table work_items alter column summary drop not null ;