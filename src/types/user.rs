pub struct User {
  pub id: String,
  pub name: String,
  pub email: String,
}

impl User {
  pub fn to_json(self) -> String {
    let template = r#"
    {
      "id": id_val,
      "name": "name_val",
      "email": "email_val"
    }
   "#;

   template.replace("name_val", &self.name).replace("email_val", &self.email).replace("id_val", &self.id)
  }
}