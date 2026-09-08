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
