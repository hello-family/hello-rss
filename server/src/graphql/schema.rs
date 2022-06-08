use async_graphql::{EmptySubscription, Schema};
use entity::async_graphql;
use migration::{Migrator, MigratorTrait};

use crate::{
    db::DB,
    graphql::{mutation::Mutation, query::Query},
};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema() -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(&DB)
        .finish()
}