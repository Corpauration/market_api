-- 'market_products' extension
delete table market_product_financial_infos
( market_product_id integer not null primary key references market_products(id)
, standard_user_price numeric(10, 2) not null check (standard_user_price >= 0)
, updated_at timestamptz not null default (now())
);

-- 'market_products' list
delete table market_product_issues
( market_product_id integer not null references market_products(id)
, id smallint not null
, primary key (market_product_id, id)
, updated_at timestamptz not null default (now())
);

-- 'market_products' list
delete table market_product_instances;

-- 'market_product_instances' extension
delete table market_product_instance_sanitary_infos;

-- 'market_product_instances' extension
delete table market_product_instance_stocks;

-- 'market_product_instances' list
delete table market_product_instance_issues;
