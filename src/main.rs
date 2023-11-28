use diesel::insert_into;
use diesel::prelude::*;

mod schema {
    diesel::table! {
        test_user {
            id -> BigInt,
            name -> Text,
            age -> Integer,
        }
    }
}

use schema::test_user;

#[derive(PartialEq, Debug)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = test_user)]
struct TestUser {
    id: i64,
    name: String,
    age: i32,
}

fn main() {
    let mut conn = init_diesel_sqlite_db();

    use self::schema::test_user::dsl::*;

    let results = test_user
        .filter(name.eq("a5"))
        // .limit(5)
        .select(TestUser::as_select());
    let results = results.load(&mut conn)
        .expect("Error loading test_user");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("id: {}, name: {}, age: {}", user.id, user.name, user.age);
    }
}

fn init_diesel_sqlite_db() -> SqliteConnection {
    let db_url = "sqlite::memory:"; // "file:test.db"
    let mut conn = SqliteConnection::establish(db_url).unwrap();
    diesel::sql_query("DROP TABLE IF EXISTS test_user;")
    .execute(&mut conn)
    .unwrap();
    
    // create table
    diesel::sql_query(
        "CREATE TABLE test_user(\
            id INTEGER PRIMARY KEY AUTOINCREMENT,\
            name VARCHAR,\
            age INTEGER\
        );"
    )
    .execute(&mut conn)
    .unwrap();

    use schema::test_user::dsl::*;

    let rst = insert_into(test_user)
        .values(&vec![
            (name.eq("a5"), age.eq(10)),
            (name.eq("a"), age.eq(20)),
            (name.eq("a"), age.eq(20)),
            (name.eq("a"), age.eq(20)),
            (name.eq("a"), age.eq(20)),
            (name.eq("a"), age.eq(20)),
            (name.eq("a"), age.eq(20)),
            (name.eq("a"), age.eq(20)),
            (name.eq("a"), age.eq(20)),
        ])
        .execute(&mut conn)
        .unwrap();

    println!("{:?}", rst);
    conn
}
