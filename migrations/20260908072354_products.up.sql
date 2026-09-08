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
