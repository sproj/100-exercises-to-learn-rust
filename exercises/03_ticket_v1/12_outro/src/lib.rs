// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
impl Order {

    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        validate_product_name(&product_name);
        validate_quantity(&quantity);
        validate_price(&unit_price);

        Order {
            product_name,
            quantity,
            unit_price
        }
    }
    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, s: String) {
        validate_product_name(&s);
        self.product_name = s;
    }

    pub fn set_quantity(&mut self, q: &u32) {
        validate_quantity(&q);
        self.quantity = *q;
    }

    pub fn set_unit_price(&mut self, p: &u32) {
        validate_price(&p);
        self.unit_price = *p;
    }
}

//   The product name can't be empty and it can't be longer than 300 bytes.
fn validate_product_name(name: &str) {
    if name.is_empty() || name.len() > 300 {
        panic!("Product name cannot be empty")
    }
}
//   The quantity must be strictly greater than zero.
fn validate_quantity(q: &u32) {
    if !(q.gt(&0)) {
        panic!("Ordered quantity cannot be zero")
    }
}
//   The unit price is in cents and must be strictly greater than zero.
fn validate_price(price: &u32) {
    if !(price.gt(&0)) {
        panic!("Unit price must be strictly greater than zero")
    }
}
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
