#[ft_sdk::form]
pub fn update_concept_status(
    mut maybe_me: backend::MaybeMe,
    ft_sdk::Json(data): ft_sdk::Json<InputData>,
) -> ft_sdk::form::Result {
    ft_sdk::println!("Updating concept status: {data:?}, {:?}", maybe_me.ud);
    data.check()?;

    let user_id = match maybe_me.ud {
        Some(ud) => ud.id,
        None => {
            ft_sdk::println!("No user data found, this should never have happened");
            return ft_sdk::form::reload();
        }
    };

    data.upsert(user_id, &mut maybe_me.conn, maybe_me.now)?;

    ft_sdk::form::reload()
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct InputData {
    concept_url: String,
    status: String,
}

impl InputData {
    fn check(&self) -> ft_sdk::Result<()> {
        if self.status != "new" && self.status != "seen" && self.status != "done" {
            return Err(ft_sdk::single_error(
                "status",
                "unknown status, only new, seen or done allowed",
            )
            .into());
        }

        Ok(())
    }

    fn upsert(
        self,
        user_id: i64,
        conn: &mut ft_sdk::Connection,
        now: chrono::DateTime<chrono::Utc>,
    ) -> ft_sdk::Result<()> {
        use backend::schema::lets_teach_user_concept;
        use diesel::prelude::*;

        diesel::insert_into(lets_teach_user_concept::table)
            .values((
                lets_teach_user_concept::user_id.eq(user_id),
                lets_teach_user_concept::concept_url.eq(&self.concept_url),
                lets_teach_user_concept::status.eq(&self.status),
                lets_teach_user_concept::created_at.eq(now),
                lets_teach_user_concept::updated_at.eq(now),
            ))
            .on_conflict((
                lets_teach_user_concept::user_id,
                lets_teach_user_concept::concept_url,
            ))
            .do_update()
            .set((
                lets_teach_user_concept::status.eq(&self.status),
                lets_teach_user_concept::updated_at.eq(now),
            ))
            .execute(conn)?;

        Ok(())
    }
}
