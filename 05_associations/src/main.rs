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
    user_id: i32,
}

impl Post {
    fn new(id: i32, title: &str, body: Option<&str>, user_id: i32) -> Self {
        Self {
            id,
            title: title.into(),
            body: body.map(Into::into),
            user_id,
        }
    }
}

#[derive(PartialEq, Debug, Queryable)]
struct User {
    id: i32,
    name: String,
}

impl User {
    fn new(id: i32, name: &str) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
}

fn load_post_and_user(conn: &PgConnection, post_id: i32) -> QueryResult<(Post, User)> {
    // Make this function load the post with the given id, along with its user
    unimplemented!();
}

fn load_all_users_and_posts(conn: &PgConnection) -> QueryResult<Vec<(User, Vec<Post>)>> {
    // Make this function return tuples of every user, and the posts belonging
    // to that user. This should be done in two queries total.
    Ok(Vec::new())
}

#[test]
fn load_using_join() -> Fallible<()> {
    let conn = connection()?;
    diesel::insert_into(users::table)
        .values(vec![
            users::name.eq("Julia"),
            users::name.eq("Jane"),
        ])
        .execute(&conn)?;
    diesel::insert_into(posts::table)
        .values(vec![
            (posts::title.eq("Julia's post"), posts::user_id.eq(1)),
            (posts::title.eq("Jane's post"), posts::user_id.eq(2)),
        ])
        .execute(&conn)?;

    let post_and_user = load_post_and_user(&conn, 1)?;
    let expected_post_and_user = (
        Post::new(1, "Julia's post", None, 1),
        User::new(1, "Julia"),
    );
    assert_eq!(expected_post_and_user, post_and_user);

    let post_and_user = load_post_and_user(&conn, 2)?;
    let expected_post_and_user = (
        Post::new(2, "Jane's post", None, 2),
        User::new(2, "Jane"),
    );
    assert_eq!(expected_post_and_user, post_and_user);

    Ok(())
}

#[test]
fn load_associated_data() -> Fallible<()> {
    let conn = connection()?;
    diesel::insert_into(users::table)
        .values(vec![
            users::name.eq("Julia"),
            users::name.eq("Jane"),
        ])
        .execute(&conn)?;
    diesel::insert_into(posts::table)
        .values(vec![
            (posts::title.eq("Julia's first post"), posts::user_id.eq(1)),
            (posts::title.eq("Jane's post"), posts::user_id.eq(2)),
            (posts::title.eq("Julia's second post"), posts::user_id.eq(1)),
        ])
        .execute(&conn)?;

    let data = load_all_users_and_posts(&conn)?;
    let expected_data = vec![
        (User::new(1, "Julia"), vec![
            Post::new(1, "Julia's first post", None, 1),
            Post::new(3, "Julia's second post", None, 1),
        ]),
        (User::new(2, "Jane"), vec![Post::new(2, "Jane's post", None, 2)]),
    ];
    assert_eq!(data, expected_data);

    Ok(())
}

fn connection() -> Fallible<PgConnection> {
    let database_url = dotenv::var("DATABASE_URL")?;
    let connection = PgConnection::establish(&database_url)?;
    connection.begin_test_transaction()?;
    diesel::sql_query("LOCK TABLE posts").execute(&connection)?;
    diesel::sql_query("LOCK TABLE users").execute(&connection)?;
    diesel::sql_query("SELECT setval('posts_id_seq', 1, false)").execute(&connection)?;
    diesel::sql_query("SELECT setval('users_id_seq', 1, false)").execute(&connection)?;
    Ok(connection)
}
