use dioxus::prelude::*;

mod error;
mod home;
mod login;
mod signup;
mod root;

pub use error::ErrorPage;
pub use home::Home;
pub use login::Login;
pub use signup::SignUp;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
 
    #[route("/login?:state")]
    Login { state: String },
    #[route("/signup?:state")]
    SignUp { state: String },
    
    #[route("/error?:message")]
    ErrorPage { message: String },
}
