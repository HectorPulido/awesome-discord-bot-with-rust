use super::super::schema::resources;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Clone, Debug, Queryable)]
pub struct Resources {
    pub id: i32,
    pub user_id: String,
    pub channel_id: String,
    pub url: String,
    pub description: String,
    pub resources_type: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "resources"]
pub struct NewResources<'a> {
    pub user_id: &'a String,
    pub channel_id: &'a String,
    pub url: &'a String,
    pub description: &'a String,
    pub resources_type: &'a i32,
}

impl Resources {
    pub fn get_random_resource(
        query: &str,
        db: &PgConnection,
    ) -> Result<Vec<Resources>, diesel::result::Error> {
        use super::super::schema::resources::dsl::*;
        use diesel::dsl::sql;
        let f: diesel::expression::SqlLiteral<i32> = sql("RANDOM()");
        return resources
            .filter(description.like(query))
            .order(f)
            .limit(1)
            .load::<Resources>(db);
    }

    pub fn create_resource<'a>(
        conn: &PgConnection,
        author_id: String,
        channel_id: String,
        url: String,
        description: String,
    ) -> Result<Resources, diesel::result::Error> {
        let data = NewResources {
            user_id: &author_id,
            channel_id: &channel_id,
            url: &url,
            description: &description,
            resources_type: &1,
        };

        return diesel::insert_into(resources::table)
            .values(data)
            .get_result(conn);
    }
}
