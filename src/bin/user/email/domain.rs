

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DomainVars {
    pub password_reset: String,
    pub url: String,
    pub support_email: String,
}
impl DomainVars {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let degen_env = match std::env::var("DEGEN_ENV").ok() {
            Some(e) => e,
            None => String::from("develop"),
        };

        if degen_env == "production" {
            let password_reset = String::from("https://www.degentracker.com/password-reset");
            let url = String::from("https://www.degentracker.com");
            let support_email = String::from("admin@degentracker.com");

            DomainVars {
                password_reset: password_reset,
                url: url,
                support_email: support_email,
            }

        } else {
            let password_reset = String::from("https://www.degentracker.com/password-reset");
            let url = String::from("https://www.degentracker.com");
            let support_email = String::from("admin@degentracker.com");

            DomainVars {
                password_reset: password_reset,
                url: url,
                support_email: support_email,
            }

        }
    }
}