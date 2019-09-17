#[macro_use]
extern crate diesel;

mod schema;

use diesel::prelude::*;
use failure::Fallible;

use crate::schema::*;

#[derive(PartialEq, Debug)]
#[derive(Queryable)]
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

#[derive(Insertable)]
#[table_name = "posts"]
struct NewPost<'a> {
    title: &'a str,
    body: &'a str,
}

fn save_new_post(conn: &PgConnection, title: &str, body: &str) -> QueryResult<()> {
    diesel::insert_into(posts::table)
        .values((
            posts::title.eq(title),
            posts::body.eq(body),
        ))
        .execute(conn)?;
    Ok(())
}

fn save_new_posts(conn: &PgConnection, new_posts: Vec<NewPost>) -> QueryResult<Vec<Post>> {
    diesel::insert_into(posts::table)
        .values(new_posts)
        .get_results(conn)
}

#[test]
fn inserting_single_record() -> Fallible<()> {
    let conn = connection()?;

    save_new_post(&conn, "Hello", "This is a post")?;
    let expected_posts = vec![
        Post::new(1, "Hello", Some("This is a post")),
    ];
    assert_eq!(expected_posts, posts::table.load(&conn)?);

    Ok(())
}

#[test]
fn inserting_many_records() -> Fallible<()> {
    let conn = connection()?;

    let posts = save_new_posts(&conn, vec![
        NewPost { title: "First Post", body: "Please Read" },
        NewPost { title: "Second Post", body: "Please Ignore" },
    ])?;
    let expected_posts = vec![
        Post::new(1, "First Post", Some("Please Read")),
        Post::new(2, "Second Post", Some("Please Ignore")),
    ];
    assert_eq!(expected_posts, posts);

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
