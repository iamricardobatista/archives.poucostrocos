use std::env;
/**
 * Returns the string value of an env variable or a default string value
 * Internaly it uses the function env::var to fetch the env var variable
 * or an error, the error is ignored and the default value is returned.
 *
 * # Examples
 *
 * ```
 * use duck_feet::utils::env;
 *
 * let result = env::get_or_else("example_env", "default");
 * assert_eq!(result, "default")
 * ```
 */
pub fn get_or_else(env: String, default: String) -> String {
    match env::var(env) {
        Ok(static_dir) => static_dir,
        Err(_) => String::from(default),
    }
}
