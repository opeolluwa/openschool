use askama::Template;

#[derive(Template)]
#[template(path = "invoice_email.html")]
pub struct InvoiceEmailTemplate<'a> {
    pub name: &'a str,
    pub identifier: &'a str,
    pub total: &'a str,
    pub due_date: &'a str,
}
