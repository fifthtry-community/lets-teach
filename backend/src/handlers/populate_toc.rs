// visit /backend/hello-world/ to see the result
#[ft_sdk::processor]
pub fn populate_toc(
    mut maybe_me: backend::MaybeMe,
    ft_sdk::Required(mut toc): ft_sdk::Required<"toc", Vec<Entry>>,
) -> ft_sdk::processor::Result {
    let ud = match maybe_me.ud {
        Some(ud) => ud.id,
        None => return ft_sdk::processor::json(toc)
    };

    fix_status(ud, &mut maybe_me.conn, &mut toc);
    ft_sdk::processor::json(toc)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
enum Status {
    New,
    Seen,
    Done,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct Entry {
    title: String,
    url: String,
    concept: Option<String>,
    status: Option<Status>,
}

fn fix_status(_user_id: i64, _conn: &mut ft_sdk::Connection, toc: &mut [Entry]) {
    let _concept_urls = toc.iter()
        .filter_map(|e| e.concept.as_ref())
        .collect::<std::collections::HashSet<_>>();

    todo!()
}
