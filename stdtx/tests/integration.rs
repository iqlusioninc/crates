//! Integration tests

use stdtx::Schema;

/// Path to an example schema TOML file
const EXAMPLE_SCHEMA: &str = "tests/support/example_schema.toml";

/// Load an example [`Schema`] from a TOML file
#[test]
fn load_schema() {
    let schema = Schema::load_toml(EXAMPLE_SCHEMA).unwrap();
    assert_eq!(schema.definitions().len(), 2);

    for definition in schema.definitions() {
        for (i, field) in definition.fields().iter().enumerate() {
            assert_eq!(i + 1, field.tag() as usize);
        }
    }
}
