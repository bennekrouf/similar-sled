use rocket::request::{Request, FromRequest, Outcome};

pub struct XHR;

impl<'a, 'r> FromRequest<'a, 'r> for XHR {
    type Error = std::convert::Infallible;

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<XHR, Self::Error> {
        if request.headers().contains("X-Requested-With") && 
           request.headers().get("X-Requested-With").any(|x| x == "XMLHttpRequest") {
            Outcome::Success(XHR)
        } else {
            Outcome::Forward(())
        }
    }
}
