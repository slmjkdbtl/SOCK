// wengwengweng

#![recursion_limit="128"]

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::DeriveInput;
use syn::Data;
use syn::Fields;

#[proc_macro_derive(Vertex, attributes(bind))]
pub fn comp_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {

	let input: DeriveInput = syn::parse(tokens).unwrap();
	let name = &input.ident;
	let data = &input.data;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let (stride, push_block, attr_block) = get_data(data);

	let expanded = quote! {

		impl #impl_generics VertexLayout for #name #ty_generics #where_clause {

			const STRIDE: usize = #stride;

			fn push(&self, queue: &mut Vec<f32>) {
				#push_block
			}

			fn attr() -> Vec<gl::VertexAttr> {

				return vec![
					#attr_block
// 					gl::VertexAttr::new(0, 2, 0),
// 					gl::VertexAttr::new(1, 2, 2),
// 					gl::VertexAttr::new(2, 4, 4),
				];

			}

		}

	};

	return proc_macro::TokenStream::from(expanded);

}

fn get_data(data: &Data) -> (TokenStream, TokenStream, TokenStream) {

	return match *data {

		Data::Struct(ref data) => {

			match data.fields {

				Fields::Named(ref fields) => {

					let mut stride = 0 as usize;
					let mut push_recurse = Vec::new();
					let mut attr_recurse = Vec::new();

					for f in &fields.named {

						let mut index = None;
						let name = &f.ident;

						for attr in f.attrs.iter() {

							for p in attr.path.segments.iter() {

								if p.ident == "bind" {

									let tts = &attr.tts;
									index = Some(tts);

								}

							}

						}

						let index = index.expect("must bind index");

						if let syn::Type::Array(arr) = &f.ty {

							if let syn::Expr::Lit(lit) = &arr.len {

								if let syn::Lit::Int(int) = &lit.lit {

									let len = int.value();

									attr_recurse.push(quote_spanned! {f.span() =>
										gl::VertexAttr::new(#index as u32, #len as i32, #stride),
									});

									stride += len as usize;

								} else {
									panic!("length has to be integer")
								}

							} else {
								panic!("length has to be literal");
							}

						} else {
							panic!("only accept fixed length arrays");
						}

						push_recurse.push(quote_spanned! {f.span() =>
							for v in &self.#name {
								queue.push(*v);
							}
						});

					}

					return (quote! {
						#stride
					}, quote!{
						#(#push_recurse)*
					}, quote!{
						#(#attr_recurse)*
					});

				},

				_ => panic!("cannot have unamed fields"),

			}

		},

		Data::Enum(_) => panic!("cannot generate vertex for enums"),
		Data::Union(_) => panic!("cannot generate vertex for unions"),

	};

}


