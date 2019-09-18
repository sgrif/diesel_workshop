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

fn remove_all_bodies(conn: &PgConnection) -> QueryResult<()> {
    diesel::update(posts::table)
        .set(posts::body.eq(None::<String>))
        .execute(conn)?;
    Ok(())
}

fn update_post_title(conn: &PgConnection, id: i32, title: &str) -> QueryResult<Post> {
    diesel::update(posts::table.find(id))
        .set(posts::title.eq(title))
        .get_result(conn)
}

fn update_post(conn: &PgConnection, post: Post) -> QueryResult<()> {
    diesel::update(&post)
        .set(&post)
        .execute(conn)?;
    Ok(())
}

#[test]
fn update_entire_table() -> Fallible<()> {
    let conn = connection()?;

    diesel::insert_into(posts::table)
        .values(vec![
            (posts::title.eq("Hi"), posts::body.eq("there")),
            (posts::title.eq("A"), posts::body.eq("post")),
        ])
        .execute(&conn)?;
    remove_all_bodies(&conn)?;

    let bodies = posts::table
        .select(posts::body)
        .load::<Option<String>>(&conn)?;
    assert_eq!(vec![None, None], bodies);

    Ok(())
}

#[test]
fn update_single_record() -> Fallible<()> {
    let conn = connection()?;

    diesel::insert_into(posts::table)
        .values((posts::title.eq("Draft Post"), posts::body.eq("My Post")))
        .execute(&conn)?;

    let post = update_post_title(&conn, 1, "Published Post")?;
    assert_eq!(Post::new(1, "Published Post", Some("My Post")), post);

    Ok(())
}

#[test]
fn update_multiple_columns() -> Fallible<()> {
    let conn = connection()?;

    let mut post = diesel::insert_into(posts::table)
        .values((posts::title.eq("Draft Post"), posts::body.eq("My Post")))
        .get_result::<Post>(&conn)?;
    post.title = String::from("Published Post");
    let post_id = post.id;

    update_post(&conn, post)?;
    let post = posts::table.find(post_id).first(&conn)?;
    assert_eq!(Post::new(1, "Published Post", Some("My Post")), post);

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
