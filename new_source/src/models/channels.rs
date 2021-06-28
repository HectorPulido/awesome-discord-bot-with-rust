use super::super::schema::channels;
use diesel::prelude::*;
use diesel::result::Error as diesel_error;

#[derive(Queryable)]
pub struct Channels {
    pub id: i32,
    pub channel_id: String,
    pub channel_type: i32,
}

#[derive(Insertable)]
#[table_name = "channels"]
pub struct NewChannels<'a> {
    pub channel_id: &'a str,
    pub channel_type: &'a i32,
}

impl Channels {
    pub fn channel_exist(db: &PgConnection, ch_id: &i32) -> bool {
        use super::super::schema::channels::dsl::*;
        let data = channels
            .filter(channel_id.eq(ch_id.to_string()))
            .limit(1)
            .load::<Channels>(db);
        match data {
            Ok(c) => return c.len() > 0,
            Err(_) => return false,
        }
    }

    pub fn get_channel(db: &PgConnection, ch_id: &i32) -> Option<Channels> {
        use super::super::schema::channels::dsl::*;
        let data = channels
            .filter(channel_id.eq(ch_id.to_string()))
            .limit(1)
            .load::<Channels>(db);
        match data {
            Ok(c) => {
                if c.len() > 0 {
                    return Some(c.into_iter().nth(0).unwrap());
                } else {
                    return None;
                }
            }
            Err(_) => return None,
        }
    }
    pub fn remove_channel(db: &PgConnection, ch_id: &u64) -> Result<usize, diesel_error> {
        let ch_id = ch_id.to_string();
        let ch_id = ch_id.as_str();
        use super::super::schema::channels::dsl::*;

        return diesel::delete(channels.filter(channel_id.eq(ch_id))).execute(db);
    }

    pub fn create_channel<'a>(
        db: &PgConnection,
        channel_id: &'a u64,
        channel_type: &'a i32,
    ) -> Result<Channels, diesel_error> {
        let channel_id = channel_id.to_string();
        let channel_id = channel_id.as_str();
        let new_type = NewChannels {
            channel_id,
            channel_type,
        };
        return diesel::insert_into(channels::table)
            .values(&new_type)
            .get_result(db);
    }
}
