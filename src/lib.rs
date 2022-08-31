use pgx::*;

pg_module_magic!();

#[pg_extern]
fn hello_daret_pg() -> &'static str {
    "Hello, daret_pg"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_daret_pg() {
        assert_eq!("Hello, daret_pg", crate::hello_daret_pg());
    }

}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
