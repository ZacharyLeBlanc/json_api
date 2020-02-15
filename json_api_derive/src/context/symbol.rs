use syn::Path;

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);
pub const ID: Symbol = Symbol("id");
pub const TO_ONE: Symbol = Symbol("to_one");
pub const URL: Symbol = Symbol("url");

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}
