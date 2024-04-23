use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};
use uuid::Uuid;

use crate::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
}

impl User {
    pub fn create(
        conn: &mut PgConnection,
        user: NewUser,
    ) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(user)
            .returning(User::as_returning())
            .get_result(conn)
    }

    pub fn find_by_email(conn: &mut PgConnection, email: &str) -> Option<User> {
        users::dsl::users
            .select(User::as_select())
            .filter(users::dsl::email.eq(email))
            .first(conn)
            .ok()
    }
}