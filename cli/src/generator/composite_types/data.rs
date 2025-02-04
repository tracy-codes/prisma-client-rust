use crate::generator::prelude::*;

pub fn struct_definition(ty: &dml::CompositeType, module_path: &TokenStream) -> TokenStream {
    let fields = ty.fields.iter().flat_map(|field| {
        let field_name_str = &field.name;
        let field_name_snake = snake_ident(&field.name);
        let field_ty = field.type_tokens(module_path)?;

        Some(quote! {
            #[serde(rename = #field_name_str)]
            pub #field_name_snake: #field_ty
        })
    });

    let specta_derive = cfg!(feature = "specta").then(|| {
        let ty_name_pascal_str = pascal_ident(&ty.name).to_string();

        quote! {
            #[derive(::prisma_client_rust::specta::Type)]
            #[specta(rename = #ty_name_pascal_str, crate = "prisma_client_rust::specta")]
        }
    });

    quote! {
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
        #specta_derive
        pub struct Data {
            #(#fields),*
        }
    }
}
