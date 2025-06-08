// visit /backend/hello-world/ to see the result
#[ft_sdk::processor]
pub fn populate_toc(
    ft_sdk::Required(mut toc): ft_sdk::Required<"toc", Vec<Entry>>,
) -> ft_sdk::processor::Result {
    ft_sdk::println!("populate_toc called with input: {toc:?}");

    toc.push(Entry {
        title: "Got data from backend".to_string(),
        url: "https://lets-teach.fifthtry.site/".to_string(),
        concept: Some("lets-teach".to_string()),
        status: Some(Status::New) });

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
