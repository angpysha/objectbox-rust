use genco::{prelude::Rust, quote, Tokens};

pub(crate) trait StringHelper {
    fn as_comma_separated_str(&self) -> Tokens<Rust>;
    fn get_id(&self) -> Tokens<Rust>;
    fn get_uid(&self) -> Tokens<Rust>;
}

impl StringHelper for String {
    fn as_comma_separated_str(&self) -> Tokens<Rust> {
        let v: Vec<&str> = self.split(":").collect();
        quote!($(v[0]), $(v[1]))
    }

    fn get_id(&self) -> Tokens<Rust> {
        let v: Vec<&str> = self.split(":").collect();
        quote!($(v[0]))
    }

    fn get_uid(&self) -> Tokens<Rust> {
        let v: Vec<&str> = self.split(":").collect();
        quote!($(v[1]))
    }
}

impl StringHelper for &str {
    fn as_comma_separated_str(&self) -> Tokens<Rust> {
        let v: Vec<&str> = self.split(":").collect();
        quote!($(v[0]), $(v[1]))
    }

    fn get_id(&self) -> Tokens<Rust> {
        let v: Vec<&str> = self.split(":").collect();
        quote!($(v[0]))
    }

    fn get_uid(&self) -> Tokens<Rust> {
        let v: Vec<&str> = self.split(":").collect();
        quote!($(v[1]))
    }
}
