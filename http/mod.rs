use std::string::String;

mod http {

	pub enum RequestType {
	  GET,
	  PUT,
	  POST,
	  DELETE
	}

	fn make_request_str(request_type: RequestType, path: &'static str) -> String {
		let mut result: String = String::new();

		result.push_str(match request_type {
			GET => "GET",
			PUT => "PUT",
			POST => "POST",
			DELETE => "DELETE"
		});

		result.push_str(" ");
		result.push_str(path);
		result.push_str(" HTTP/1.0\r\n\r\n");
		result.to_string()
	}
		
	#[test]
	fn test_make_request_str() {
		let request_type = GET;
		let path = "/hello/world";
		let result = make_request_str(request_type, path);
		let expected = "GET /hello/world HTTP/1.0\r\n\r\n".to_string();
		println!("Result is: {}", result);
		assert!(result == expected);
	}

}