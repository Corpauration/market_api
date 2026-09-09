use serde::{ Serialize, Deserialize };
use chrono::{ DateTime, Utc };
use rust_decimal::Decimal;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Users {
    pub id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserDisplayInfos {
    pub user_id: i32,
    pub display_name: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserContactInfos {
    pub user_id: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserContactInfoEmailAddresses {
    pub user_id: i32,
    pub email_address_id: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct EmailAddresses {
    pub id: i32,
    pub email_address: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct EmailAddressDisablements {
    pub email_address_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UsersEmailAddresses {
    pub user_id: i32,
    pub email_address_id: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct CyTechStudentNumbers {
    pub id: i32,
    pub cy_tech_student_number: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UsersCyTechStudentNumbers {
    pub user_id: i32,
    pub cy_tech_student_number_id: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct AuthenticationRights {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserAuthenticationPorts {
    pub user_id: i32,
    pub id: i16,
    pub name: String,
    pub visible: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserAuthenticationPortRights {
    pub user_id: i32,
    pub port_id: i16,
    pub right_id: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserCredentials {
    pub user_id: i32,
    pub id: i16,
    pub name: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserPasswordCredentials {
    pub user_id: i32,
    pub credential_id: i16,
    pub password_hash: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserEmailAddressCredentials {
    pub user_id: i32,
    pub credential_id: i16,
    pub email_address_id: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserAuthenticationPortCredentials {
    pub user_id: i32,
    pub port_id: i16,
    pub credential_id: i16,
    pub visible: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserAuthenticationPortRequiredPorts {
    pub user_id: i32,
    pub port_id: i16,
    pub required_port_id: i16,
    pub visible: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserIdentifiers {
    pub user_id: i32,
    pub id: i16,
    pub name: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserEmailAddressIdentifiers {
    pub user_id: i32,
    pub id: i16,
    pub email_address_id: i32,
    pub discriminator: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserCyTechStudentNumberIdentifiers {
    pub user_id: i32,
    pub id: i16,
    pub cy_tech_student_number_id: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MandateTitles {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserMandates {
    pub user_id: i32,
    pub id: i16,
    pub title: i32,
    pub effective_date: DateTime<Utc>,
    pub expiration_date: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MarketProducts {
    pub id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MarketProductDisplayInfos {
    pub market_product_id: i32,
    pub display_name: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MarketProductFinancialInfos {
    pub market_product_id: i32,
    pub standard_user_price: Decimal,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MarketProductIssues {
    pub market_product_id: i32,
    pub id: i16,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MarketProductInstances {
    pub market_product_id: i32,
    pub id: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MarketProductInstanceSanitaryInfos {
    pub market_product_id: i32,
    pub market_product_instance_id: i32,
    pub serial_number: String,
    pub expiration_date: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MarketProductInstanceStocks {
    pub market_product_id: i32,
    pub market_product_instance_id: i32,
    pub quantity: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MarketProductInstanceIssues {
    pub market_product_id: i32,
    pub market_product_instance_id: i32,
    pub id: i16,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MarketRestocks {
    pub id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MarketRestockOwners {
    pub market_restock_id: i32,
    pub user_id: i32,
    pub mandate_id: i16,
    pub execution_date: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MarketRestockProducts {
    pub market_restock_id: i32,
    pub market_product_id: i32,
    pub market_product_instance_id: i32,
    pub quantity: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MarketOrders {
    pub user_id: i32,
    pub id: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MarketOrderProducts {
    pub user_id: i32,
    pub market_order_id: i32,
    pub market_product_id: i32,
    pub market_product_instance_id: i32,
    pub quantity: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MarketOrderFinancialInfos {
    pub user_id: i32,
    pub market_order_id: i32,
    pub total_price: Decimal,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MarketOrderCashPayments {
    pub user_id: i32,
    pub market_order_id: i32,
    pub id: i16,
    pub amount: Decimal,
    pub payment_date: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MarketOrderCashPaymentWitnesses {
    pub user_id: i32,
    pub market_order_id: i32,
    pub market_order_cash_payment_id: i16,
    pub witness_user_id: i32,
    pub witness_mandate_id: i16,
    pub updated_at: DateTime<Utc>,
}
