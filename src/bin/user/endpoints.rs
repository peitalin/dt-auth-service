

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Endpoint<'a> {
    /// "Base" endpoint URL is set for:
    /// 1. GraphiQL Playground
    /// 2. Email verification and resets
    /// or any HTML templates that refer back to api endpoints
    /// from the end user's perspective.
    /// e.g: https://api.fileworks.net/user
    /// It is set with the ENDPOINT environment variable
    Base(&'a str),
    Payment(&'a str),
    User(&'a str),
    Content(&'a str),
    Upload(&'a str),
    Gateway(&'a str),
    Shopping(&'a str),
    Notify(&'a str),
}
impl<'a> Endpoint<'a> {
    pub fn as_url(&self) -> String {
        format!("{}", self)
    }
    pub fn as_path(&self) -> &str {
        match *self {
            Endpoint::Base(path) => path,
            Endpoint::Payment(path) => path,
            Endpoint::User(path) => path,
            Endpoint::Content(path) => path,
            Endpoint::Upload(path) => path,
            Endpoint::Gateway(path) => path,
            Endpoint::Shopping(path) => path,
            Endpoint::Notify(path) => path,
        }
    }
}
impl<'a> From<Endpoint<'a>> for String {
    fn from(e: Endpoint) -> String {
        e.as_url()
    }
}
impl<'a> std::fmt::Display for Endpoint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // e.g: 0.0.0.0:8082
        let endpoint = match *self {
            Endpoint::Base(path) => format_endpoint("ENDPOINT", path),
            Endpoint::Payment(path) => format_endpoint("GUN_PAYMENT_URL", path),
            Endpoint::User(path) => format_endpoint("GUN_USER_URL", path),
            Endpoint::Content(path) => format_endpoint("GUN_CONTENT_URL", path),
            Endpoint::Upload(path) => format_endpoint("GUN_UPLOAD_URL", path),
            Endpoint::Gateway(path) => format_endpoint("GUN_GATEWAY_URL", path),
            Endpoint::Shopping(path) => format_endpoint("GUN_SHOPPING_URL", path),
            Endpoint::Notify(path) => format_endpoint("GUN_NOTIFY_URL", path),
        };
        write!(f, "{}", endpoint)
    }
}

fn format_endpoint(var: &str, path: &str) -> String {
    use std::env;
    dotenv::dotenv().ok();
    let uri = trim_trailing_slash(env::var(var))
            .expect(&format!("{} .env var to be set", var));
    format!("{}{}", uri, check_leading_slash(path))
}

use std::env::VarError;
fn trim_trailing_slash(
    var: Result<String, VarError>
) -> Result<String, VarError> {
    match var {
        Err(e) => Err(e),
        Ok(mut s) => {
            if s.ends_with("/") {
                s.pop();
                Ok(s)
            } else {
                Ok(s)
            }
        }
    }
}

fn check_leading_slash(s: &str) -> String {
    if s.starts_with("/") {
        s.to_string()
    } else {
        format!("/{}", s)
    }
}

mod tests {
    use super::*;

    #[test]
    fn check_leading_slash_exists() {
        let test1 = check_leading_slash("some/route");
        assert_eq!(
            test1,
            String::from("/some/route")
        );
    }

    #[test]
    fn check_leading_slash_exists2() {
        let test1 = check_leading_slash("/some/route");
        assert_eq!(test1, String::from("/some/route"));
    }

    #[test]
    fn check_trailing_slash_removed1() {
        let test1 = trim_trailing_slash(Ok(String::from("/some/route/")));
        assert_eq!(test1, Ok(String::from("/some/route")));
    }

    #[test]
    fn check_trailing_slash_removed2() {
        let test1 = trim_trailing_slash(Ok(String::from("/some/route")));
        assert_eq!(test1, Ok(String::from("/some/route")));
    }

    #[test]
    fn format_endpoint_without_leading_slash() {

        use std::env;
        dotenv::dotenv().ok();

        let uri = env::var("ENDPOINT").expect("ENDPOINT .env var to be set");
        let test1 = format_endpoint("ENDPOINT", "/some/route");

        assert_eq!(
            test1,
            format!("{}{}", uri, "/some/route")
        );
    }

    #[test]
    fn format_endpoint_with_leading_slash() {

        use std::env;
        dotenv::dotenv().ok();

        let uri = env::var("ENDPOINT").expect("ENDPOINT .env var to be set");
        let test1 = format_endpoint("ENDPOINT", "some/route");

        assert_eq!(
            test1,
            format!("{}{}", uri, "/some/route")
        );
    }

}
