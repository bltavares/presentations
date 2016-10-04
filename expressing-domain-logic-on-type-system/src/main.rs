mod lib {
    pub struct SessionToken(String);

    pub struct InvalidOrder(String);

    pub enum ApiError {
        ParsingError(String),
        IoError(String),
    }

    pub struct Order {
        amount: u8,
        name: String,
    }

    pub struct OrderResponse {
        pub name: String,
        pub status: OrderStatus,
        pub amount: u8,
    }

    pub enum OrderStatus {
        Waiting,
        Shipping,
        Shipped,
        Delivered,
    }

    pub type ApiResponse<T> = Result<T, ApiError>;

    pub fn create_order(amount: u8, name: String) -> Result<Order, InvalidOrder> {
        if amount <= 0 {
            unimplemented!()
        }
        unimplemented!()
    }

    pub fn authorize(auth_token: String) -> ApiResponse<SessionToken> {
        unimplemented!()
    }

    pub fn send_order(session_token: &SessionToken, order: Order) -> ApiResponse<OrderResponse> {
        unimplemented!()
    }
}

pub use lib::*;

fn main() {
    if let Ok(session_token) = authorize("My initial token".into()) {
        let first_order = create_order(10, "Bananas".into());

        if let Ok(order) = first_order {
            send_order(&session_token, order);
        }
    }
}
