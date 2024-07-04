use std::fmt::{Display, Result};

#[warn(unused_imports)]
pub struct Applicant<'a> {
    name: String,
    phone: Phone,
    rut: Rut<'a>,
    address: Address,
    email: Email,
}

pub struct Phone(Option<u8>, u32);
impl Phone {
    fn new(code: Option<u8>, body: u32) -> Self {
        let country_code: u8 = {
            match code {
                Some(number) => number,
                None => 56u8,
            }
        };
        Self(Some(country_code), body)
    }
}
impl Display for Phone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "+{}{}", &self.0.unwrap(), &self.1)
    }
}

pub struct Rut<'a>(u32, &'a str);

pub struct Email {}
pub struct Address {
    city: String,
    region: String,
    street: Option<String>,
    number: Option<u8>,
}
