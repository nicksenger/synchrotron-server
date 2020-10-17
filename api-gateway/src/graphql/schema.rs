use juniper::{EmptyMutation, EmptySubscription};

use super::query::Query;
use crate::data::{UserData};

#[derive(Clone)]
pub struct Context {
    pub user_data: UserData,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(
        user_data: UserData,
    ) -> Self {
        Self {
            user_data
        }
    }
}

pub type Schema =
    juniper::RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(
        Query {},
        juniper::EmptyMutation::new(),
        juniper::EmptySubscription::new(),
    )
}
