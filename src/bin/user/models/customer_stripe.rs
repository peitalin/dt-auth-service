use dt::utils::dates::from_datetimestr_to_naivedatetime;
use dt::utils::dates::from_timestamp_s_to_naivedatetime;




/// The resource representing a Stripe customer.
/// For more details see https://stripe.com/docs/api#customers.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomerStripeCompact {
    /// CustomerId, e.g: cus_FAi7syjFdLL0Qu
    pub id: String,

    /// An integer amount in cents that represents the customer’s current balance
    pub balance: i32,

    /// Unix Timestamp
    #[serde(deserialize_with = "from_timestamp_s_to_naivedatetime")]
    pub created: chrono::NaiveDateTime,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// When the customer's latest invoice is billed by charging automatically, delinquent is true if the invoice's latest charge is failed.
    ///
    /// When the customer's latest invoice is billed by sending an invoice, delinquent is true if the invoice is not paid by its due date.
    #[serde(default)]
    pub delinquent: bool,

    /// Livemode or Testmode
    pub livemode: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,

    /// An integer amount in cents that represents the customer’s current balance
    /// Renamed to `balance`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<i32>,

    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// Business VAT id number for Euro zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_vat_id: Option<String>,

    /// Currency, e.g.: "aud", "usd", "gbp", "eur"
    pub currency: Option<String>,

    /// Default PaymentMethod or Source ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<String>,

    ///An arbitrary string that you can attach to a customer object.
    /// It is displayed alongside the customer in the dashboard.
    /// This will be unset if you POST an empty value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Customer's email address. This may be up to 512 characters.
    /// This will be unset if you POST an empty value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    // /// A set of key-value pairs that you can attach to a customer object.
    // /// Must be a String that deserializes into a HashMap<String, String>
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub metadata: Option<String>,

    /// The customer’s full name or business name.
    /// This will be unset if you POST an empty value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The customer’s phone number. This will be unset if you POST an empty value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Customer’s preferred languages, ordered by preference.
    /// This will be unset if you POST an empty value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,
}


#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Address {
    pub city: Option<String>,
    pub country: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
    /// The town/cho-me (Japan only)
    pub town: Option<String>,
}




#[test]
fn deserializes_stripe_customer_response() {
    let test_str = r#"
        {
            "id": "cus_H8BMIf0Q6aXNnd",
            "object": "customer",
            "account_balance": 0,
            "address": null,
            "balance": 0,
            "created": 1587371351,
            "currency": null,
            "default_source": null,
            "delinquent": false,
            "description": null,
            "discount": null,
            "email": "peita+2@protonmail.com",
            "invoice_prefix": "6732DF3B",
            "invoice_settings": {
                "custom_fields": null,
                "default_payment_method": null,
                "footer": null
            },
            "livemode": false,
            "metadata": {  },
            "name": "J P",
            "next_invoice_sequence": 1,
            "phone": null,
            "preferred_locales": [ ],
            "shipping": null,
            "sources": {
                "object": "list",
                "data": [ ],
                "has_more": false,
                "total_count": 0,
                "url": "/v1/customers/cus_H8BMIf0Q6aXNnd/sources"
            },
            "subscriptions": {
                "object": "list",
                "data": [],
                "has_more": false,
                "total_count": 0,
                "url": "/v1/customers/cus_H8BMIf0Q6aXNnd/subscriptions"
            },
            "tax_exempt": "none",
            "tax_ids": {
                "object": "list",
                "data": [],
                "has_more": false,
                "total_count": 0,
                "url": "/v1/customers/cus_H8BMIf0Q6aXNnd/tax_ids"
            },
            "tax_info": null,
            "tax_info_verification": null
        }
    "#;

    let res = serde_json::from_str::<CustomerStripeCompact>(test_str);
    match res {
        Ok(c) => {
            assert_eq!(c.id, String::from("cus_H8BMIf0Q6aXNnd"));
            debug!("created at: {:?}", &c.created);
            assert_eq!(
                c.created,
                chrono::NaiveDateTime::from_timestamp(1_587_371_351, 0)
            );
        },
        Err(e) => panic!(format!("{:?}", e)),
    }
}

