-- Identity
drop table users;

-- Identity
drop table email_addresses;

-- Identity
drop table cy_tech_student_numbers;

-- Identity
drop table market_restocks;

-- Identity
drop table market_products;

-- 'users' extension
drop table user_display_infos;

-- 'users' extension
drop table user_contact_infos;

-- 'users', 'email_addresses' matrix
drop table users_email_addresses;

-- 'user_contact_infos' list
drop table user_contact_info_email_addresses;

-- 'email_addresses' list
drop table email_address_disablements;

-- 'users', 'cy_tech_student_numbers' matrix
drop table users_cy_tech_student_numbers;

-- Enum
drop table authentication_rights;
--copy authentication_rights (title)
--from 'enums/authentication_rights.csv'
--with (format csv, header true);

-- 'users' list
drop table user_authentication_ports;

-- 'user_authentication_ports' list
drop table user_authentication_port_rights;

-- 'users' list
drop table user_credentials;

-- 'user_credentials' variant
drop table user_password_credentials;

-- 'user_credentials' variant
drop table user_email_address_credentials;

-- 'user_authentication_ports' list
drop table user_authentication_port_credentials;

-- 'user_authentication_ports' list
drop table user_authentication_port_required_ports;

-- 'users' list
drop table user_identifiers;

-- 'user_identifiers' variant
drop table user_email_address_identifiers;

-- 'user_identifiers' variant
drop table user_cy_tech_student_number_identifiers;

-- Enum
drop table mandate_titles;
--copy mandate_titles (title)
--from 'enums/mandate_titles.csv'
--with (format csv, header true);

-- 'users' list
drop table user_mandates;

-- 'market_products' extension
drop table market_product_display_infos;

-- 'market_products' extension
drop table market_product_financial_infos;

-- 'market_products' list
drop table market_product_issues;

-- 'market_products' list
drop table market_product_instances;

-- 'market_product_instances' extension
drop table market_product_instance_sanitary_infos;

-- 'market_product_instances' extension
drop table market_product_instance_stocks;

-- 'market_product_instances' list
drop table market_product_instance_issues;

-- 'market_restocks', 'user_mandates' matrix
drop table market_restock_owners;

-- 'market_restocks', 'market_product_instances' matrix
drop table market_restock_products;

-- 'users' list
drop table market_orders;

-- 'market_orders', 'market_product_instances' matrix
drop table market_order_products;

-- 'market_orders' extension
drop table market_order_financial_infos;

-- 'market_order' list
drop table market_order_cash_payments;

-- 'market_order_cash_payments', 'user_mandates' matrix
drop table market_order_cash_payment_witnesses;
