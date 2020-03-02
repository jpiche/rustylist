-- Your SQL goes here
create table tasks (
  id bigserial primary key,
  user_id bigint not null,
  title varchar not null,
  description varchar null,
  created_at timestamp with time zone not null default now(),
  updated_at timestamp with time zone not null default now()
);

create table users (
  id BIGSERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  created_at timestamp with time zone not null default now(),
  updated_at timestamp with time zone not null default now()
);

create index on tasks (user_id);
