#[cfg(test)]
mod tests {
    extern crate requests;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert_eq!(3 * 6, 18);
    }
    #[test]
    fn requests() {
        let response = requests::get("http://httpbin.org/get").unwrap();
        assert_eq!(response.status_code(), requests::StatusCode::Ok);
    }
}
