use syn::Path;

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);
pub const ID: Symbol = Symbol("id");

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}
