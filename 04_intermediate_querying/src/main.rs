#[macro_use]
extern crate diesel;

mod schema;

use diesel::prelude::*;
use failure::Fallible;

use crate::schema::*;

#[derive(PartialEq, Debug)]
#[derive(Queryable, Identifiable, AsChangeset)]
struct Post {
    id: i32,
    title: String,
    body: Option<String>,
}

impl Post {
    fn new(id: i32, title: &str, body: Option<&str>) -> Self {
        Self {
            id,
            title: title.into(),
            body: body.map(Into::into),
        }
    }
}

fn posts_about_rust(conn: &PgConnection) -> QueryResult<Vec<Post>> {
    // Make this function return all posts that contain "Rust" in the title,
    // ordered alphabetically
    Ok(Vec::new())
}

#[test]
fn filtering_and_ordering() -> Fallible<()> {
    let conn = connection()?;
    diesel::insert_into(posts::table)
        .values(vec![
            posts::title.eq("Rust is my love"),
            posts::title.eq("A sonnet for Go"),
            posts::title.eq("A ballad for Rust"),
        ])
        .execute(&conn)?;

    let posts = posts_about_rust(&conn)?;
    let expected_posts = vec![
        Post::new(3, "A ballad for Rust", None),
        Post::new(1, "Rust is my love", None),
    ];
    assert_eq!(posts, expected_posts);

    Ok(())
}

fn connection() -> Fallible<PgConnection> {
    let database_url = dotenv::var("DATABASE_URL")?;
    let connection = PgConnection::establish(&database_url)?;
    connection.begin_test_transaction()?;
    diesel::sql_query("LOCK TABLE posts").execute(&connection)?;
    diesel::sql_query("SELECT setval('posts_id_seq', 1, false)").execute(&connection)?;
    Ok(connection)
}
