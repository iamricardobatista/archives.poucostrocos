use std::env;

/// Returns the string value of an env variable or a default string value
/// Internaly it uses the function env::var to fetch the env var variable
/// or an error, the error is ignored and the default value is returned.
///
/// # Examples
///
/// ```
/// use poucostrocos::utils::env;
///
/// let result = env::get_or_else(String::from("example_env"), String::from("default"));
/// assert_eq!(result, "default")
/// ```
///
/// ```
/// use poucostrocos::utils::env;
///
/// std::env::set_var("example_env", "example_value");
///
/// let result = env::get_or_else(String::from("example_env"), String::from("default"));
/// assert_eq!(result, "example_value")
/// ```
pub fn get_or_else(env: String, default: String) -> String {
    match env::var(env) {
        Ok(value) => value,
        Err(_) => String::from(default),
    }
}

/// Returns the string value of an env variable or panics
///
/// # Examples
///
/// ```
/// use poucostrocos::utils::env;
///
/// std::env::set_var("example_env", "example_value");
///
/// let result = env::get("example_env");
/// assert_eq!(result, "example_value")
/// ```
///
/// This function panics when the env var doesn't exists
///
/// # panics
///
/// ```rust,should_panic
/// let result = env::get("example_env");
/// ```
pub fn get(env: String) -> String {
    match env::var(env) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    }
}
