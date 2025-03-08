use serde_json::Value;
use fake::{
  Fake,
  faker::{
    boolean::raw::*,
    internet::raw::*,
    lorem::raw::*,
    name::raw::*,
  },
  locales::EN
};
use diesel::prelude::*;
use std::string::String;
use std::collections::HashMap;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use diesel::result::Error;
use rand::Rng;

use crate::models::{Post, User};
use crate::interfaces::{NewPost, NewUser};

fn create_post(
  conn: &mut PgConnection, 
  title: &str, 
  body: &str,
  published: &bool,
  published_by_id: &i32,
  metadata: &Value,
) -> Post {
  use crate::schema::posts;

  let new_post = NewPost { 
    title, 
    body,
    published,
    published_by_id,
    metadata,
  };

  diesel::insert_into(posts::table)
      .values(&new_post)
      .returning(Post::as_returning())
      .get_result(conn)
      .expect("Error saving new post")
}

fn create_user(
  conn: &mut PgConnection, 
  first_name: &str,
  last_name: &str,
  email: &str,
  password: &str,
) -> User {
  use crate::schema::users;
  
  let new_user = NewUser { 
    first_name, 
    last_name,
    email,
    password,
  };

  diesel::insert_into(users::table)
      .values(&new_user)
      .returning(User::as_returning())
      .get_result(conn)
      .expect("Error saving new user")
}

fn generate_and_insert_users(conn: &mut PgConnection, num_users: u64) -> Vec<User> {
  let pb = ProgressBar::new(num_users);
  pb.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
    .unwrap()
    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"));

  let mut users = Vec::new();
  for _ in 0..num_users {
    let email = SafeEmail(EN).fake::<String>();
    let password = Password(EN, 12..15).fake::<String>();
    users.push(
      create_user(
        conn,
        FirstName(EN).fake(),
        LastName(EN).fake(),
        &String::from(email),
        &password,
      )
    );
    pb.inc(1);
  }
  pb.finish_with_message("Users generated");
  users
}

pub fn generate_and_insert_posts_for_user(conn: &mut PgConnection, user: User, num_posts: u64, pb: &ProgressBar) -> Vec<Post> {
  let colors = vec!["red", "blue", "green", "yellow", "pink", "orange", "brown", "magenta"];
  let mut posts = Vec::new();
  for _ in 0..num_posts {
    let mut metadata = HashMap::new();
    for color in colors.iter() {
      let data = Words(EN, 15..20).fake::<Vec<String>>();
      metadata.entry(color).or_insert(data);
    }

    let title = Sentence(EN, 10..15).fake::<String>();
    let body = Paragraphs(EN, 7..10).fake::<Vec<String>>();
    let published = Boolean(EN, 50).fake::<bool>();
    let metadata_string = serde_json::to_string(&metadata).unwrap();

    posts.push(
      create_post(
        conn,
        &title,
        &body.join("\n"),
        &published,
        &user.id,
        &(serde_json::from_str(&metadata_string).unwrap()),
      )
    );
    pb.inc(1);
  }
  posts
}

pub fn get_random_users(conn: &mut PgConnection, num: u64) -> Vec<User> {
  use crate::schema::users::dsl::*;

  let min_id: Option<i32> = users.select(diesel::dsl::min(id)).first(conn).unwrap();
  let max_id: Option<i32> = users.select(diesel::dsl::max(id)).first(conn).unwrap();

  if min_id.is_none() || max_id.is_none() {
    return Vec::new();
  }

  let min_id = min_id.unwrap();
  let max_id = max_id.unwrap();

  let mut rng = rand::rng();
  let mut random_users = Vec::new();

  while random_users.len() < num as usize {
    let random_id = rng.random_range(min_id..=max_id);
    if let Ok(user) = users.filter(id.eq(random_id)).first::<User>(conn) {
      random_users.push(user);
    }
  }

  random_users
}

pub fn generate_data(conn: &mut PgConnection, create_new_users: bool) -> Result<(), Error> {
  const NUM_USERS: u64 = 3;
  const NUM_POSTS_PER_USER: u64 = 10000;

  let pb_posts = ProgressBar::new(NUM_POSTS_PER_USER * NUM_USERS);
  pb_posts.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
    .unwrap()
    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"));

  conn.transaction::<_, Error, _>(|conn| {
    let users = if create_new_users {
      generate_and_insert_users(conn, NUM_USERS)
    } else {
      get_random_users(conn, NUM_USERS)
    };
    for user in users {
      generate_and_insert_posts_for_user(conn, user, NUM_POSTS_PER_USER, &pb_posts);
    }
    pb_posts.finish_with_message("Posts generated");

    Ok(())
  })
}
