// visit /backend/hello-world/ to see the result
#[ft_sdk::processor]
pub fn populate_toc(
    mut maybe_me: backend::MaybeMe,
    ft_sdk::Required(mut toc): ft_sdk::Required<"toc", Vec<Entry>>,
    ft_sdk::Required(current_url): ft_sdk::Required<"current-url">,
) -> ft_sdk::processor::Result {
    let concept_url = toc
        .iter()
        .find(|e| e.url == current_url && e.concept.is_some())
        .and_then(|e| e.concept.clone());

    let ud = match maybe_me.ud {
        Some(ud) => ud.id,
        None => {
            ft_sdk::println!("No user data found, returning toc without status");
            return ft_sdk::processor::json(PageData { toc, concept_url });
        }
    };

    fix_status(ud, &mut maybe_me.conn, &mut toc)?;
    ft_sdk::processor::json(PageData { toc, concept_url })
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct PageData {
    toc: Vec<Entry>,
    concept_url: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
enum Status {
    New,
    Seen,
    Done,
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        match s {
            "new" => Status::New,
            "seen" => Status::Seen,
            "done" => Status::Done,
            _ => unreachable!("Unknown status: {}", s),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct Entry {
    title: String,
    url: String,
    concept: Option<String>,
    status: Option<Status>,
}

fn fix_status(
    user_id: i64,
    conn: &mut ft_sdk::Connection,
    toc: &mut [Entry],
) -> ft_sdk::Result<()> {
    use backend::schema::lets_teach_user_concept;
    use diesel::prelude::*;

    let concept_urls = toc
        .iter()
        .filter_map(|e| e.concept.as_ref())
        .collect::<std::collections::HashSet<_>>();

    let uc_list = lets_teach_user_concept::table
        .filter(lets_teach_user_concept::user_id.eq(user_id))
        .filter(lets_teach_user_concept::status.ne("new"))
        .filter(lets_teach_user_concept::concept_url.eq_any(concept_urls))
        .select(backend::schema::UC::as_select())
        .load(conn)?;

    let mut uc_map = std::collections::HashMap::new();
    for uc in uc_list {
        uc_map.insert(uc.concept_url, uc.status);
    }

    for entry in toc {
        if let Some(concept) = &entry.concept {
            if let Some(status) = uc_map.get(concept) {
                entry.status = Some(status.as_str().into());
            } else {
                entry.status = Some(Status::New);
            }
        } else {
            entry.status = None;
        }
    }

    Ok(())
}
