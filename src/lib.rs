#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Token, parse_macro_input, AttributeArgs, BareFnArg, Ident, ItemFn, ItemType, Type, TypeBareFn, punctuated::Punctuated, spanned::Spanned};

#[proc_macro_attribute]
#[doc = include_str!("../README.md")]
pub fn type_alias(args: TokenStream, input: TokenStream) -> TokenStream {
	let args = parse_macro_input!(args as AttributeArgs);
	assert!(args.len() != 0, "Please provide a name for the type alias");
	assert!(args.len() <= 2, "Please only provide an optional visibility specifier plus a name for the type alias");

	let function = parse_macro_input!(input as ItemFn);
	let type_alias_function = {
		let function = function.clone();
		TypeBareFn {
			lifetimes: None,
			unsafety: function.sig.unsafety,
			abi: function.sig.abi,
			fn_token: function.sig.fn_token,
			paren_token: function.sig.paren_token,
			inputs: {
				let mut punct = Punctuated::new();
				for arg in function.sig.inputs {
					punct.push(match arg {
						syn::FnArg::Receiver(receiver) => {
							let arg = quote!(#receiver.self_type).into();
							let mut arg = parse_macro_input!(arg as BareFnArg);
							arg.attrs = receiver.attrs;
							arg
						},
						syn::FnArg::Typed(arg) => BareFnArg {
							attrs: arg.attrs,
							name: {
								let pat = arg.pat.into_token_stream().into();
								Some((parse_macro_input!(pat as Ident), arg.colon_token))
							},
							ty: *arg.ty
						},
					});
				}
				punct
			},
			variadic: function.sig.variadic,
			output: function.sig.output,
		}
	};

	let mut args = args.into_iter();
	let (vis, type_alias_name) = match (args.next(), args.next()) {
		(Some(ident), None) => {
			let ident = ident.into_token_stream().into();
			let type_alias_name = parse_macro_input!(ident as syn::Ident);
			(None, type_alias_name)
		},
		(Some(vis), Some(ident)) => {
			let vis = vis.into_token_stream().into();
			let ident = ident.into_token_stream().into();
			let vis = parse_macro_input!(vis as syn::Visibility);
			let type_alias_name = parse_macro_input!(ident as syn::Ident);
			(Some(vis), type_alias_name)
		},
		_ => panic!("Please provide a name and/or visibility for the type alias"),
	};

	let type_alias = ItemType {
		attrs: vec![],
		vis: vis.unwrap_or(function.vis.clone()),
		type_token: Token![type](function.span()),
		ident: type_alias_name,
		generics: function.sig.generics.clone(),
		eq_token: Token![=](function.span()),
		ty: Box::new(Type::BareFn(type_alias_function)),
		semi_token: Token![;](function.span()),
	};

	quote!(#type_alias #function).into()
}
