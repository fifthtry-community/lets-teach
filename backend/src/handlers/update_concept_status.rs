#[ft_sdk::form]
pub fn update_concept_status(
    maybe_me: backend::MaybeMe,
    ft_sdk::Json(data): ft_sdk::Json<ConceptStatus>,
) -> ft_sdk::form::Result {
    ft_sdk::println!("Updating concept status: {data:?}, {:?}", maybe_me.ud);
    ft_sdk::form::reload()
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ConceptStatus {
    concept_url: String,
    status: String,
}
