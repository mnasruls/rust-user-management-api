-- Your SQL goes here
create table users (
  id            uuid primary key default gen_random_uuid(),
  name          varchar(250)    not null,
  email         varchar(250)    not null,
  password      text            not null,
  role_id       uuid            not null references roles(id),
  created_at    timestamptz     not null default now(),
  updated_at    timestamptz     not null default now(),
  deleted_at    timestamptz
);