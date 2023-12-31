-- Add migration script here
-- project table.
alter table projects alter column created_on type timestamptz ;
alter table projects alter column updated_on type timestamptz ;

-- work_items table.
alter table work_items alter column created_on type timestamptz ;
alter table work_items alter column completed_on type timestamptz ;