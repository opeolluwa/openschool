use askama::Template;

#[derive(Template)]
#[template(path = "invoice.html")]
pub struct InvoiceTemplate {
    pub identifier: String,
    pub recipient_name: String,
    pub recipient_email: String,
    pub recipient_phone: String,
    pub invoice_date: String,
    pub due_date: String,
    pub currency: String,
    pub items: Vec<InvoiceLineItem>,
    pub subtotal: String,
    pub total: String,
    pub total_in_words: String,
    pub authorized_by: String,
}

pub struct InvoiceLineItem {
    pub index: usize,
    pub product_name: String,
    pub product_description: String,
    pub unit_price: String,
    pub quantity: i32,
    pub line_total: String,
}
