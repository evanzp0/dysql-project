use sea_orm::{EntityTrait, QueryFilter, Condition, ColumnTrait};
use seaorm::test_user::{Entity as TestUser, self};

#[tokio::main]
async fn main() {
    // let db = sea_orm::Database::connect("postgres://root:111111@localhost/my_database").await.unwrap();
    let db = init_seaorm_connection().await;
 
    let user = TestUser::find()
        .filter(
            Condition::all()
                .add(test_user::Column::Name.eq("a5"))
        )
        .one(&db).await.unwrap();

    println!("{:#?}", user);

}

async fn init_seaorm_connection() -> sea_orm::DatabaseConnection {
    use sea_orm::ConnectionTrait;
    use sea_orm::Set;

    let db = sea_orm::Database::connect("sqlite::memory:").await.unwrap();
    let builder = db.get_database_backend();
    let schema = sea_orm::Schema::new(builder);
    // builder.build(&schema.create_table_from_entity(TestUser));
    let table_create_statement = schema.create_table_from_entity(TestUser);
    let _ = db.execute(builder.build(&table_create_statement)).await;

    let _ = TestUser::insert_many(
        vec![
            test_user::ActiveModel { id: Set(1), name: Set(Some("a5".to_owned())), age: Set(Some(10)) },
            test_user::ActiveModel { id: Set(2), name: Set(Some("a".to_owned())), age: Set(Some(10)) },
            test_user::ActiveModel { id: Set(3), name: Set(Some("a".to_owned())), age: Set(Some(10)) },
            test_user::ActiveModel { id: Set(4), name: Set(Some("a".to_owned())), age: Set(Some(10)) },
            test_user::ActiveModel { id: Set(5), name: Set(Some("a".to_owned())), age: Set(Some(10)) },
            test_user::ActiveModel { id: Set(6), name: Set(Some("a".to_owned())), age: Set(Some(10)) },
            test_user::ActiveModel { id: Set(7), name: Set(Some("a".to_owned())), age: Set(Some(10)) },
            test_user::ActiveModel { id: Set(8), name: Set(Some("a".to_owned())), age: Set(Some(10)) },
            test_user::ActiveModel { id: Set(9), name: Set(Some("a".to_owned())), age: Set(Some(10)) },
        ]
    )
    .exec(&db)
    .await
    .unwrap();

    db
}