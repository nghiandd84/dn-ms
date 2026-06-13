#[cfg(test)]
mod tests {
    use features_auth_model::client::ClientData;
    use serde_json;

    #[test]
    fn client_secret_is_not_exposed_in_json_response() {
        let mut client = ClientData::default();
        client.client_secret = Some("super_secret_value".to_string());

        let json = serde_json::to_string(&client).unwrap();
        assert!(
            !json.contains("client_secret"),
            "client_secret field should not appear in serialized JSON"
        );
        assert!(
            !json.contains("super_secret_value"),
            "client_secret value should not appear in serialized JSON"
        );
    }
}
