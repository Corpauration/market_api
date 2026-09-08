-- Identity
create table email_addresses
( id integer primary key generated always as identity
, email_address text not null unique
, created_at timestamptz not null default (now())
);

-- 'email_addresses' list
create table email_address_disablements
( email_address_id integer not null primary key references email_addresses(id)
, created_at timestamptz not null default (now())
);

-- 'users', 'email_addresses' matrix
create table users_email_addresses
( user_id integer not null references users(id)
, email_address_id integer not null references email_addresses(id)
, primary key (user_id, email_address_id)
, updated_at timestamptz not null default (now())
);

