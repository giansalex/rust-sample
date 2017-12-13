extern crate requests;

use requests::ToJson;

pub fn run() {
    let response = requests::post("http://httpbin.org/post").unwrap();
    assert_eq!(response.status_code(), requests::StatusCode::Ok);

    let data = response.json().unwrap();

    assert_eq!(data["url"], "http://httpbin.org/post");
    assert_eq!(data["headers"]["Host"], "httpbin.org");

    println!("Url {}", data["url"]);
}
