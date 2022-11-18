use std::time::Instant;

use sysinfo::System;
use tokio::sync::RwLock;

pub struct State {
    pub uptime: Instant,
    pub system: RwLock<System>,
}

pub type Context<'a> = poise::Context<'a, State, anyhow::Error>;
