-- Enum
create table mandate_titles
( id integer primary key generated always as identity
, title text not null unique
);
copy mandate_titles (title)
from 'enums/mandate_titles.csv'
with (format csv, header true);

-- 'users' list
create table user_mandates
( user_id integer not null references users(id)
, id smallint not null
, primary key (user_id, id)
, title integer not null references mandate_titles(id)
, effective_date timestamptz not null
, expiration_date timestamptz not null
, check (effective_date <= expiration_date)
, updated_at timestamptz not null default (now())
);
