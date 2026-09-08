-- 'users' list
delete table market_orders;

-- 'market_orders', 'market_product_instances' matrix
delete table market_order_products;

-- 'market_orders' extension
delete table market_order_financial_infos;

-- 'market_order' list
delete table market_order_cash_payments;

-- 'market_order_cash_payments', 'user_mandates' matrix
delete table market_order_cash_payment_witnesses;
