-- Add migration script here
-- Create subscriptions table
create table subscriptions(
       id uuid NOT NULL,
       primary key (id),
       email text not null unique,
       name text not null,
       subscribed_at timestamptz not null
);
