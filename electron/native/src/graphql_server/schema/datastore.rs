use i54_::i54;
use std::convert::TryInto;

#[derive(juniper::GraphQLObject, turbosql::Turbosql, Default)]
pub struct Pdf {
 pub rowid: Option<i54>,
 pub id: Option<i32>,
 pub filesize: Option<i54>,
 pub name: Option<String>,
 pub content: Option<String>,
}

pub fn add_pdf(content: &str) -> anyhow::Result<Pdf> {
 let pdf = Pdf {
  content: Some(content.to_string()),
  name: Some(format!("a file of {} bytes", content.len())),
  filesize: Some(content.len().try_into()?),
  ..Default::default()
 };

 pdf.insert()?;

 Ok(pdf)
}

pub fn list_pdfs() -> turbosql::Result<Vec<Pdf>> {
 turbosql::select!(Vec<Pdf>)
}
