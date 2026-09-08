-- Enum
create table authentication_rights
( id integer primary key generated always as identity
, title text not null unique
);
copy authentication_rights (title)
from 'enums/authentication_rights.csv'
with (format csv, header true);

-- 'users' list
create table user_authentication_ports
( user_id integer not null references users(id)
, id smallint not null
, primary key (user_id, id)
, name text not null
, unique (user_id, name)
, visible boolean not null default true
, updated_at timestamptz not null default (now())
);

-- 'user_authentication_ports' list
create table user_authentication_port_rights
( user_id integer not null
, port_id smallint not null
, foreign key (user_id, port_id) references user_authentication_ports(user_id, id)
, right_id integer not null references authentication_rights(id)
, primary key (user_id, port_id, right_id)
, updated_at timestamptz not null default (now())
);

-- 'users' list
create table user_credentials
( user_id integer not null references users(id)
, id smallint not null
, primary key (user_id, id)
, name text not null
, unique (user_id, name)
, updated_at timestamptz not null default (now())
);

-- 'user_credentials' variant
create table user_password_credentials
( user_id integer not null
, credential_id smallint not null
, foreign key (user_id, credential_id) references user_credentials(user_id, id)
, primary key (user_id, credential_id)
, password_hash text not null
, updated_at timestamptz not null default (now())
);

-- 'user_credentials' variant
create table user_email_address_credentials
( user_id integer not null
, credential_id smallint not null
, foreign key (user_id, credential_id) references user_credentials(user_id, id)
, primary key (user_id, credential_id)
, email_address_id integer not null
, foreign key (user_id, email_address_id) references users_email_addresses(user_id, email_address_id)
, updated_at timestamptz not null default (now())
);

-- 'user_authentication_ports' list
create table user_authentication_port_credentials
( user_id integer not null
, port_id smallint not null
, foreign key (user_id, port_id) references user_authentication_ports(user_id, id)
, credential_id smallint not null
, foreign key (user_id, credential_id) references user_credentials(user_id, id)
, primary key (user_id, port_id, credential_id)
, visible boolean not null default true
, updated_at timestamptz not null default (now())
);

-- 'user_authentication_ports' list
create table user_authentication_port_required_ports
( user_id integer not null
, port_id smallint not null
, foreign key (user_id, port_id) references user_authentication_ports(user_id, id)
, required_port_id smallint not null
, foreign key (user_id, required_port_id) references user_authentication_ports(user_id, id)
, primary key (user_id, port_id, required_port_id)
, visible boolean not null default true
, updated_at timestamptz not null default (now())
);

-- 'users' list
create table user_identifiers
( user_id integer not null references users(id)
, id smallint not null
, primary key (user_id, id)
, name text not null
, unique (user_id, name)
, updated_at timestamptz not null default (now())
);

-- 'user_identifiers' variant
create table user_email_address_identifiers
( user_id integer not null
, id smallint not null
, foreign key (user_id, id) references user_identifiers(user_id, id)
, primary key (user_id, id)
, email_address_id integer not null
, foreign key (user_id, email_address_id) references users_email_addresses(user_id, email_address_id)
, discriminator text not null
, unique (email_address_id, discriminator)
, updated_at timestamptz not null default (now())
);

-- 'user_identifiers' variant
create table user_cy_tech_student_number_identifiers
( user_id integer not null
, id smallint not null
, foreign key (user_id, id) references user_identifiers(user_id, id)
, primary key (user_id, id)
, cy_tech_student_number_id integer not null unique
, foreign key (user_id, cy_tech_student_number_id) references users_cy_tech_student_numbers(user_id, cy_tech_student_number_id)
, updated_at timestamptz not null default (now())
);
