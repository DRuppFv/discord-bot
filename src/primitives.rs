pub struct State {}

pub type Context<'a> = poise::Context<'a, State, anyhow::Error>;
