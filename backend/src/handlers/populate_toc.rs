// visit /backend/hello-world/ to see the result
#[ft_sdk::processor]
pub fn populate_toc(
) -> ft_sdk::processor::Result {
    // lets-teach.entry list
    ft_sdk::processor::json(vec![Entry {
        title: "Lets Teach".to_string(),
        url: "https://lets-teach.fifthtry.site/".to_string(),
        concept: Some("lets-teach".to_string()),
        status: Some(Status::New),
    }, Entry {
        title: "Lets Teach - Concepts".to_string(),
        url: "https://lets-teach.fifthtry.site/concepts/".to_string(),
        concept: Some("lets-teach-concepts".to_string()),
        status: Some(Status::Seen),
    }, Entry {
        title: "Lets Teach - Concepts - Statuses".to_string(),
        url: "https://lets-teach.fifthtry.site/concepts/statuses/".to_string(),
        concept: Some("lets-teach-concepts-statuses".to_string()),
        status: Some(Status::Done),
    }])
}


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