use std::string::String;

mod http {

	macro_rules! cat(
	    ($inp:ident, $($sp:ident),+) => ({
	    	$(
	            $inp.push_str($sp);
	        )+
	    });
	)

	pub enum RequestType {
	  GET,
	  PUT,
	  POST,
	  DELETE
	}

	fn make_request_str(request_type: RequestType, path: &'static str) -> String {
		let mut result: String = String::new();
		let suffix = " HTTP/1.0\r\n\r\n";

		result.push_str(match request_type {
			GET => "GET ",
			PUT => "PUT ",
			POST => "POST ",
			DELETE => "DELETE "
		});

		cat!(result, path, suffix);
		result.to_string()
	}
		
	#[test]
	fn test_make_request_str() {
		let request_type = GET;
		let result = make_request_str(request_type, "/hello/world");
		println!("Result is: {}", result);

		let expected = "GET /hello/world HTTP/1.0\r\n\r\n".to_string();
		assert!(result == expected);
	}

}