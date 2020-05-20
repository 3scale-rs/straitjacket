/// Note: this is deprecated.
struct AccessToken {
    user_id: String,
    name: String,
    permission: String, // "ro" or "rw"
    scopes: Option<Vec<String>>, // ary of url-encoded, for master: ["account_management", "stats"], for tenant, the same plus "finance"
    access_token: String, // the token
}
