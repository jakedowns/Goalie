#[test]
fn test_add_score() {
    let client = reqwest::blocking::Client::new();
    let mutation = r#"
        mutation AddScore {
            addScore(input: { gameId: "some-game-id", userId: "some-user-id", value: 42 }) {
                id
            }
        }
    "#;
    let response = client
        .post("http://localhost:8000/graphql")
        .json(&json!({ "query": mutation }))
        .send()
        .unwrap();
    assert!(response.status().is_success());
    let body = response.json::<serde_json::Value>().unwrap();
    assert_eq!(body["data"]["addScore"]["id"], "some-score-id");
}
