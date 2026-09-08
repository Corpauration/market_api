-- Identity
create table users
( id integer primary key generated always as identity
, created_at timestamptz not null default (now())
);

-- 'users' extension
create table user_display_infos
( user_id integer not null primary key references users(id)
, display_name text not null
, updated_at timestamptz not null default (now())
);

-- 'users' extension
create table user_contact_infos
( user_id integer not null primary key references users(id)
, updated_at timestamptz not null default (now())
);

-- 'user_contact_infos' list
create table user_contact_info_email_addresses
( user_id integer not null
, foreign key (user_id) references user_contact_infos(user_id)
, email_address_id integer not null
, foreign key (user_id, email_address_id) references users_email_addresses(user_id, email_address_id)
, primary key (user_id, email_address_id)
, updated_at timestamptz not null default (now())
);
