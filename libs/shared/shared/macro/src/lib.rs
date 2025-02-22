use proc_macro::TokenStream;

mod builder;
mod dto;
mod filter;
mod mutation;
mod proc_example;
mod query;
mod response;

#[proc_macro]
pub fn make_answer(input: TokenStream) -> TokenStream {
    proc_example::make_answer(input)
}

#[proc_macro_derive(Query, attributes(query_filter, query))]
pub fn query_derive(input: TokenStream) -> TokenStream {
    query::query_impl(input)
}

#[proc_macro_derive(Mutation, attributes(mutation))]
pub fn mutation_derive(input: TokenStream) -> TokenStream {
    mutation::mutation_impl(input)
}

#[proc_macro_derive(Greet)]
pub fn greet_macro_derive(input: TokenStream) -> TokenStream {
    proc_example::greet_macro_derive(input)
}

#[proc_macro_attribute]
pub fn log_function_name(_attr: TokenStream, item: TokenStream) -> TokenStream {
    proc_example::log_function_name(_attr, item)
}

#[proc_macro_derive(Builder, attributes(builder_metadata))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    proc_example::derive_builder(input)
}

#[proc_macro_derive(ParamFilter)]
pub fn param_filter_macro_derive_impl(input: TokenStream) -> TokenStream {
    filter::filter_macro_derive_impl(input)
}

#[proc_macro_derive(Response)]
pub fn response_json(input: TokenStream) -> TokenStream {
    response::response_json(input)
}

#[proc_macro_derive(ResponseGeneric)]
pub fn response_json_generic(input: TokenStream) -> TokenStream {
    response::response_json_generic(input)
}

#[proc_macro_derive(Dto, attributes(dto))]
pub fn derive_dto(input: TokenStream) -> TokenStream {
    dto::derive_dto(input)
}
