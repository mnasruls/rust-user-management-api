-- Your SQL goes here
create table roles (
  id            uuid primary key default gen_random_uuid(),
  name          varchar(250)    not null,
  code          varchar(250)    not null,
  description   text            not null,
  created_at    timestamptz     not null default now(),
  updated_at    timestamptz     not null default now(),
  deleted_at    timestamptz
);