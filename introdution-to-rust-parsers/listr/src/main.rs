extern crate hyper;

use hyper::client::Client;

fn main() {
    let client = Client::new();
    let request = client.get("http://www.amazon.com.br/registry/wishlist/3DA4I0ZLH8ADW/ref=cm_sw_r_tw_ws_9hJzwb06V29HS");
    let request_result = request.send();
    let response = request_result.expect("The HTTP request failed to be made");

    println!("Response status: {}", response.status);
}
