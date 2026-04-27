use proc_macro::TokenStream;

mod dto;
mod filter;
mod mutation;
mod query;
mod response;
mod service;

#[proc_macro_derive(Query, attributes(query_filter, query, query_related))]
pub fn query_derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);
    let query_input = query::QueryInput::parse_from(derive_input);
    query::query_impl(query_input)
}

#[proc_macro_derive(Mutation, attributes(mutation))]
pub fn mutation_derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);
    let mutation_input = mutation::MutationInput::parse_from(derive_input);
    mutation::mutation_impl(mutation_input)
}

#[proc_macro_derive(RemoteService, attributes(remote))]
pub fn remote_service_macro_derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);
    let service_input = service::RemoteServiceInput::parse_from(derive_input);
    service::remote_service(service_input)
}

#[proc_macro_derive(ParamFilter, attributes(skip_param))]
pub fn param_filter_macro_derive_impl(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);
    let filter_input = filter::FilterInput::parse_from(derive_input);
    filter::filter_macro_derive_impl(filter_input)
}

#[proc_macro_derive(Response)]
pub fn response_json(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);
    let response_input = response::ResponseInput::parse_from(derive_input);
    response::response_json(response_input)
}

#[proc_macro_derive(ResponseGeneric)]
pub fn response_json_generic(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);
    let response_input = response::ResponseInput::parse_from(derive_input);
    response::response_json_generic(response_input)
}

#[proc_macro_derive(Dto, attributes(dto))]
pub fn derive_dto(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);
    let dto_input = dto::DtoInput::parse_from(derive_input);
    dto::derive_dto(dto_input)
}
