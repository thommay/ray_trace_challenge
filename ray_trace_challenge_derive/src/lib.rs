use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Groupable)]
pub fn groupable_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_groupable_macro(&ast)
}

fn impl_groupable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {

        impl<'a> Groupable<'a> for #name<'a> {
            fn set_parent(&mut self, parent: &Rc<RefCell<Group<'a>>>) {
                let parent = Rc::clone(parent);
                self.parent = Some(parent);
            }
        }
    };
    gen.into()
}
