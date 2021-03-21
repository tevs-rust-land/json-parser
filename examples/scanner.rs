fn main() {
    let json_string = r#"
        {
            "accounts": [
                {
                    "id": 35,
                    "name": "Personal",
                    "organization_name": "Individual",
                    "organization_id": 35,
                    "account_balance": 19
                },
                {
                    "id": 36,
                    "name": "Account Team",
                    "organization_name": "Uber Eats",
                    "organization_id": 36,
                    "account_balance": 19
                }
            ]
        }
        "#;
    let (tokens, scanner_errors) = json_type_parser::scanner::scan(&json_string);
    println!("-----Tokens----");
    println!("{:?}", tokens);
    println!("-----Scanner Errors----");
    println!("{:?}", scanner_errors);
}
