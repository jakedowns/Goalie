#[cfg(test)]
mod tests {
    use super::schema::Schema;
    use juniper::graphql_object;

    #[test]
    fn test_query() {
        let query = r#"
            query {
                myQuery {
                    id
                    name
                }
            }
        "#;

        let expected_result = r#"
            {
                "data": {
                    "myQuery": {
                        "id": "123",
                        "name": "John"
                    }
                }
            }
        "#;

        let schema = Schema::new(Query, Mutation, EmptySubscription);
        let result = schema.execute(query).unwrap().data;

        assert_eq!(serde_json::to_string(&result).unwrap(), expected_result);
    }

    #[test]
    fn test_mutation() {
        let mutation = r#"
            mutation {
                myMutation(input: { name: "Jane" }) {
                    id
                    name
                }
            }
        "#;

        let expected_result = r#"
            {
                "data": {
                    "myMutation": {
                        "id": "456",
                        "name": "Jane"
                    }
                }
            }
        "#;

        let schema = Schema::new(Query, Mutation, EmptySubscription);
        let result = schema.execute(mutation).unwrap().data;

        assert_eq!(serde_json::to_string(&result).unwrap(), expected_result);
    }

    #[test]
    fn test_error_handling() {
        let query = r#"
            query {
                myQuery {
                    id
                    invalid_field
                }
            }
        "#;

        let schema = Schema::new(Query, Mutation, EmptySubscription);
        let result = schema.execute(query).unwrap_err();

        assert!(result.to_string().contains("Cannot query field"));
    }

    #[test]
    fn test_performance() {
        // Perform a large number of queries/mutations to test performance
    }

    #[derive(juniper::GraphQLObject)]
    struct MyQuery {
        id: String,
        name: String,
    }

    struct Query;

    #[graphql_object]
    impl Query {
        fn my_query() -> MyQuery {
            MyQuery {
                id: "123".to_string(),
                name: "John".to_string(),
            }
        }
    }

    struct Mutation;

    #[graphql_object]
    impl Mutation {
        fn my_mutation(input: MyInput) -> MyQuery {
            MyQuery {
                id: "456".to_string(),
                name: input.name,
            }
        }
    }

    struct MyInput {
        name: String,
    }
}
