pub fn authorize(auth_token: String) -> String {
    unimplemented!()
}

pub fn send_order(session_token: String,
                  amount: u8,
                  product: String) {
    unimplemented!()
}

fn main() {
    let session_token = authorize("My initial token".into());
    send_order(session_token, 10, "Bananas".into())
}
