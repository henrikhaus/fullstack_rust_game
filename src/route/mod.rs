use actix_web::web;

pub mod user;

/// Controls all HTTP methods around a single URL
pub trait Controller {
  /// Configures all routes belonging to the controller
  fn configure(cgf: &mut web::ServiceConfig);
}
