use gotham::middleware::{NewMiddleware, Middleware};
use gotham::state::{FromState, State, StateData};
use gotham::handler::HandlerFuture;

#[derive(StateData)]
pub struct Config {
  config_file: String
}

impl Config {
  pub fn new(config_file: String) -> Config {
    Config { config_file: config_file }
  }

  pub fn get_config_file<'a>(&'a self) -> &'a str {
    &self.config_file
  }
}

impl Clone for Config {
  fn clone(&self) -> Self {
    Config {
      config_file: self.config_file.clone()
    }
  }
}

#[derive(Clone, NewMiddleware)]
pub struct ConfigMiddleware {
  config: Config
}

impl ConfigMiddleware {
  pub fn new(config: Config) -> ConfigMiddleware {
    ConfigMiddleware { config: config }
  }
}

impl Middleware for ConfigMiddleware {
  fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
  {
    state.put(self.config);

    chain(state)
  }
}