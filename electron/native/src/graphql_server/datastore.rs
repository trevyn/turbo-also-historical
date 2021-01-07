use std::convert::TryInto;
use turbosql::i53;

#[derive(juniper::GraphQLObject, turbosql::Turbosql, Default)]
pub struct Pdf {
 #[graphql(skip)]
 pub rowid: Option<i64>,
 pub id: Option<i32>,
 pub filesize: Option<i53>,
 pub name: Option<String>,
 pub content: Option<String>,
}

pub fn add_pdf(content: &str) {
 Pdf {
  content: Some(content.to_string()),
  name: Some(format!("a file of {} bytes", content.len())),
  filesize: Some(content.len().try_into().unwrap()),
  ..Default::default()
 }
 .insert()
 .unwrap();
}

pub fn list_pdfs() -> turbosql::Result<Vec<Pdf>> {
 turbosql::select!(Vec<Pdf>)
}
