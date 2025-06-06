// visit /backend/hello-world/ to see the result
#[ft_sdk::processor]
pub fn populate_toc(
    ft_sdk::Query(toc_input): ft_sdk::Query<"toc">,
) -> ft_sdk::processor::Result {
    let mut toc = serde_json::from_str::<Vec<Entry>>(&toc_input)?;

    toc.push(Entry {
        title: "Got data from backend".to_string(),
        url: "https://lets-teach.fifthtry.site/".to_string(),
        concept: Some("lets-teach".to_string()),
        status: Some(Status::New) });
    // lets-teach.entry list

    ft_sdk::processor::json(toc)
}


// #[derive(serde::Deserialize, Debug)]
// struct PopulateTocInput {
//     toc: Vec<Entry>
// }
//
// -- record entry:
// caption title-main:
// string url:
// ;; if this is set, this is an actual concept page, else it just a regular page
// optional string concept:
// ;; can be NULL, new, seen, done.
// optional string status:


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
