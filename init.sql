
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

-- Identity
create table email_addresses
( id integer primary key generated always as identity
, email_address text not null unique
, created_at timestamptz not null default (now())
);

-- 'email_addresses' list
create table email_address_disablement
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

-- Identity
create table cy_tech_student_numbers
( id integer primary key generated always as identity
, cy_tech_student_number text not null unique
, created_at timestamptz not null default (now())
);

-- 'users', 'cy_tech_student_numbers' matrix
create table users_cy_tech_student_numbers
( user_id integer not null references users(id)
, cy_tech_student_number_id integer not null references cy_tech_student_numbers(id)
, primary key (user_id, cy_tech_student_number_id)
, updated_at timestamptz not null default (now())
, unique (user_id) -- One person may only have one student number.
, unique (cy_tech_student_number_id) -- One student number may only be associated with one person.
);

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

-- Identity
create table market_products
( id integer primary key generated always as identity
, created_at timestamptz not null default (now())
);

-- 'market_products' extension
create table market_product_display_infos
( market_product_id integer not null primary key references market_products(id)
, display_name text not null
, updated_at timestamptz not null default (now())
);

-- 'market_products' extension
create table market_product_financial_infos
( market_product_id integer not null primary key references market_products(id)
, standard_user_price numeric(10, 2) not null check (standard_user_price >= 0)
, updated_at timestamptz not null default (now())
);

-- 'market_products' list
create table market_product_issues
( market_product_id integer not null references market_products(id)
, id smallint not null
, primary key (market_product_id, id)
, updated_at timestamptz not null default (now())
);

-- 'market_products' list
create table market_product_instances
( market_product_id integer not null references market_products(id)
, id integer not null
, primary key (market_product_id, id)
, updated_at timestamptz not null default (now())
);

-- 'market_product_instances' extension
create table market_product_instance_sanitary_infos
( market_product_id integer not null
, market_product_instance_id integer not null
, foreign key (market_product_id, market_product_instance_id) references market_product_instances(market_product_id, id)
, primary key (market_product_id, market_product_instance_id)
, serial_number text not null
, expiration_date timestamptz null
, updated_at timestamptz not null default (now())
);

-- 'market_product_instances' extension
create table market_product_instance_stocks
( market_product_id integer not null
, market_product_instance_id integer not null
, foreign key (market_product_id, market_product_instance_id) references market_product_instances(market_product_id, id)
, primary key (market_product_id, market_product_instance_id)
, quantity integer not null check (quantity >= 0)
, updated_at timestamptz not null default (now())
);

-- 'market_product_instances' list
create table market_product_instance_issues
( market_product_id integer not null
, market_product_instance_id integer not null
, foreign key (market_product_id, market_product_instance_id) references market_product_instances(market_product_id, id)
, id smallint not null
, primary key (market_product_id, market_product_instance_id, id)
, updated_at timestamptz not null default (now())
);

-- Identity
create table market_restocks
( id integer primary key generated always as identity
, created_at timestamptz not null default (now())
);

-- 'market_restocks', 'user_mandates' matrix
create table market_restock_owners
( market_restock_id integer not null references market_restocks(id)
, user_id integer not null
, mandate_id smallint not null
, foreign key (user_id, mandate_id) references user_mandates(user_id, id)
, primary key (market_restock_id, user_id, mandate_id)
, execution_date timestamptz not null
, updated_at timestamptz not null default (now())
);

-- 'market_restocks', 'market_product_instances' matrix
create table market_restock_products
( market_restock_id integer not null references market_restocks(id)
, market_product_id integer not null
, market_product_instance_id integer not null
, foreign key (market_product_id, market_product_instance_id) references market_product_instances(market_product_id, id)
, primary key (market_restock_id, market_product_id, market_product_instance_id)
, quantity integer not null check (quantity > 0)
, updated_at timestamptz not null default (now())
);

-- 'users' list
create table market_orders
( user_id integer not null references users(id)
, id integer not null
, primary key (user_id, id)
, updated_at timestamptz not null default (now())
);

-- 'market_orders', 'market_product_instances' matrix
create table market_order_products
( user_id integer not null
, market_order_id integer not null
, foreign key (user_id, market_order_id) references market_orders(user_id, id)
, market_product_id integer not null
, market_product_instance_id integer not null
, foreign key (market_product_id, market_product_instance_id) references market_product_instances(market_product_id, id)
, primary key (user_id, market_order_id, market_product_id, market_product_instance_id)
, quantity integer not null check (quantity > 0)
, updated_at timestamptz not null default (now())
);

-- 'market_orders' extension
create table market_order_financial_infos
( user_id integer not null
, market_order_id integer not null
, foreign key (user_id, market_order_id) references market_orders(user_id, id)
, primary key (user_id, market_order_id)
, total_price numeric(10, 2) not null check (total_price >= 0)
, updated_at timestamptz not null default (now())
);

-- 'market_order' list
create table market_order_cash_payments
( user_id integer not null
, market_order_id integer not null
, foreign key (user_id, market_order_id) references market_orders(user_id, id)
, id smallint not null
, primary key (user_id, market_order_id, id)
, amount numeric(10, 2) not null check (amount >= 0)
, payment_date timestamptz not null
, updated_at timestamptz not null default (now())
);

-- 'market_order_cash_payments', 'user_mandates' matrix
create table market_order_cash_payment_witnesses
( user_id integer not null
, market_order_id integer not null
, market_order_cash_payment_id smallint not null
, foreign key (user_id, market_order_id, market_order_cash_payment_id) references market_order_cash_payments(user_id, market_order_id, id)
, witness_user_id integer not null
, witness_mandate_id smallint not null
, foreign key (witness_user_id, witness_mandate_id) references user_mandates(user_id, id)
, primary key (user_id, market_order_id, market_order_cash_payment_id, witness_user_id, witness_mandate_id)
, updated_at timestamptz not null default (now())
);
