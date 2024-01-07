-- Add migration script here
alter table public.work_items
    add time_spent_hours float;

