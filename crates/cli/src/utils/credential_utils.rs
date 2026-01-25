use rpassword::prompt_password;

pub fn get_user_credentials_input() -> String {
    let credential =
        prompt_password("Enter Credential: ").expect("Failed to read credential from input");

    credential
}
