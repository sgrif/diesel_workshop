#[macro_use]
extern crate diesel;

mod schema;

use diesel::prelude::*;
use failure::Fallible;

#[derive(PartialEq, Debug)]
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

fn get_all_post_titles(conn: &PgConnection) -> QueryResult<Vec<String>> {
    Ok(Vec::new())
}

fn get_all_post_data(conn: &PgConnection) -> QueryResult<Vec<(i32, String, Option<String>)>> {
    Ok(Vec::new())
}

fn get_all_posts(conn: &PgConnection) -> QueryResult<Vec<Post>> {
    Ok(Vec::new())
}

#[test]
fn loading_single_column() -> Fallible<()> {
    let conn = connection()?;
    diesel::sql_query("INSERT INTO posts (title) VALUES ('My First Post'), ('My Second Post')")
        .execute(&conn)?;

    let expected_titles = vec![
        String::from("My First Post"),
        String::from("My Second Post"),
    ];
    let actual_titles = get_all_post_titles(&conn)?;

    assert_eq!(expected_titles, actual_titles);

    Ok(())
}

#[test]
fn loading_a_tuple() -> Fallible<()> {
    let conn = connection()?;
    diesel::sql_query("INSERT INTO posts (id, title, body) VALUES \
                      (1, 'My First Post', 'It''s a good post'), \
                      (2, 'My Second Post', NULL)")
        .execute(&conn)?;

    let expected_posts = vec![
        (1, String::from("My First Post"), Some(String::from("It's a good post"))),
        (2, String::from("My Second Post"), None)
    ];
    let actual_posts = get_all_post_data(&conn)?;

    assert_eq!(expected_posts, actual_posts);

    Ok(())
}

#[test]
fn loading_a_struct() -> Fallible<()> {
    let conn = connection()?;
    diesel::sql_query("INSERT INTO posts (id, title, body) VALUES \
                      (1, 'My First Post', 'It''s a good post'), \
                      (2, 'My Second Post', NULL)")
        .execute(&conn)?;

    let expected_posts = vec![
        Post::new(1, "My First Post", Some("It's a good post")),
        Post::new(2, "My Second Post", None)
    ];
    let actual_posts = get_all_posts(&conn)?;

    assert_eq!(expected_posts, actual_posts);

    Ok(())
}

fn connection() -> Fallible<PgConnection> {
    let database_url = dotenv::var("DATABASE_URL")?;
    let connection = PgConnection::establish(&database_url)?;
    connection.begin_test_transaction()?;
    Ok(connection)
}

