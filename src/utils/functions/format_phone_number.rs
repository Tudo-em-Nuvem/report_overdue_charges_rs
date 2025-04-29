pub fn format_phone_number(ddd: String, numero: String) -> String {
  format!("{}{}", ddd, numero.replace("-", ""))
      .replace(" ", "")
      .replace("(", "")
      .replace(")", "")
      .trim()
      .to_string()
}