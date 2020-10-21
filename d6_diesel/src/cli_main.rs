extern crate d6_doodle;

use clap::{clap_app, crate_version};
use d6_doodle::{models::*, schema::*};
use diesel::prelude::*;

fn main() -> Result<(), failure::Error> {
    let clap = clap_app!(doodle_cli =>
      (about: "A cli for the doodle database app")
      (version:crate_version!()) (author: "Max")
      (@subcommand new_user =>
        (@arg name:+required "the name of the new user")
        (@arg password:+required "the new user's password")
      )
      (@subcommand view_users =>)
      (@subcommand new_poll =>
        (@arg question:+required "the question")
        (@arg options:+required "the options (Comma separated)")
        (@arg user_id:-u +takes_value +required "the owner of the poll")
      )
      (@subcommand view_polls =>)
      (@subcommand respond_poll=>
        (@arg user_id:+required "User ID")
        (@arg poll_id:+required "Poll ID")
        (@arg selection:+required "Your selection as an int 0..")
      )
      (@subcommand poll_results =>
        (@arg poll_id:+required)
      )

    )
    .get_matches();
    let conn = d6_doodle::create_connection()?;
    if let Some(sub) = clap.subcommand_matches("new_user") {
        let user = new_user(
            sub.value_of("name").unwrap(),
            sub.value_of("password").unwrap(),
        );
        let added: User = diesel::insert_into(users::table)
            .values(&user)
            .get_result(&conn)?;

        println!("{:?}", added)
    }
    if let Some(_sub) = clap.subcommand_matches("view_users") {
        let res: Vec<User> = users::table.limit(10).load::<User>(&conn)?;
        for user in res {
            println!("{:?}", user);
        }
        println!("view users")
    }
    if let Some(sub) = clap.subcommand_matches("new_poll") {
        let owner: Option<i32> = match sub.value_of("user_id") {
            Some(v) => Some(v.parse()?),
            None => None,
        };

        let question = sub.value_of("question").unwrap();
        let options = sub.value_of("options").unwrap();

        let added: Poll = diesel::insert_into(polls::table)
            .values(&NewPoll {
                owner,
                question,
                options,
            })
            .get_result(&conn)?;

        println!("Added Poll: {:?}", added)
    }
    if let Some(_sub) = clap.subcommand_matches("view_users") {
        let res: Vec<Poll> = polls::table.limit(10).load::<Poll>(&conn)?;
        for poll in res {
            println!("{:?}", poll);
        }
        println!("view users")
    }

    if let Some(ref sub) = clap.subcommand_matches("respond_poll") {
        let new_response = Response {
            poll_id: sub.value_of("poll_id").unwrap().parse()?,
            user_id: sub.value_of("user_id").unwrap().parse()?,
            selected: Some(sub.value_of("selection").unwrap().parse()?),
        };
        let added: Response = diesel::insert_into(responses::table)
            .values(&new_response)
            .get_result(&conn)?;
        println!("Added response = {:?}", added)
    }

    if let Some(ref sub) = clap.subcommand_matches("poll_results") {
        let id: i32 = sub.value_of("poll_id").unwrap().parse()?;
        let poll: Poll = polls::table.find(id).first::<Poll>(&conn)?;
        let vals = responses::table
            .inner_join(users::table)
            .inner_join(polls::table)
            .select((users::id, responses::selected))
            .filter(polls::id.eq(id))
            .load::<(i32, Option<i32>)>(&conn)?;
        println!("{:?}", poll.question);
        for v in vals {
            println!("{:?}", v.1)
        }
    }

    Ok(())
}
