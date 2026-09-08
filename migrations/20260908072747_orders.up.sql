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
