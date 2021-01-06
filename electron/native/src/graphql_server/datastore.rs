#[derive(juniper::GraphQLObject, turbosql::Turbosql, Default)]
pub struct Pdf {
 #[graphql(skip)]
 pub rowid: Option<i64>,
 pub id: Option<i32>,
 pub name: Option<String>,
}
