use language::*;

#[test]
fn test_import() {
    assert_eq!(import_db("db.jql").len() > 0, true);
}
