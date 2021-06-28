use super::super::schema::types;
use diesel::prelude::*;
use diesel::result::Error as diesel_error;

#[derive(Queryable)]
pub struct Types {
    pub id: i32,
    pub type_description: String,
}

#[derive(Insertable)]
#[table_name = "types"]
pub struct NewType<'a> {
    pub type_description: &'a str,
}

impl Types {
    pub fn get_types(db: &PgConnection) -> Result<Vec<Types>, diesel_error> {
        use super::super::schema::types::dsl::*;
        return types.load::<Types>(db);
    }

    pub fn type_exist(db: &PgConnection, type_name: &str) -> Option<Types> {
        use super::super::schema::types::dsl::*;
        let data = types
            .filter(type_description.eq(type_name))
            .limit(1)
            .load::<Types>(db);

        match data {
            Ok(types_data) => {
                if types_data.len() > 0 {
                    return Some(types_data.into_iter().nth(0).unwrap());
                } else {
                    return None;
                }
            }
            Err(_) => return None,
        }
    }

    pub fn create_type<'a>(
        db: &PgConnection,
        type_description: &'a str,
    ) -> Result<Types, diesel_error> {
        let type_description = type_description.to_lowercase();
        let type_description = type_description.trim();

        let new_type = NewType { type_description };
        return diesel::insert_into(types::table)
            .values(&new_type)
            .get_result(db);
    }
}
