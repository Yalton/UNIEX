use axum::extract::FromRef;

use crate::services::*;
use crate::setup::{Args, Cfg};

#[derive(Debug, Clone)]
pub struct State {
    browser: BrowserService,
    title: TextExtractor,
}

impl State {
    pub async fn new(_args: &Args, cfg: &Cfg) -> anyhow::Result<Self> {
        let browser = BrowserService::connect(&cfg.browser).await?;
        let title = TextExtractor::new(&cfg.openapi)?;

        Ok(Self { browser, title })
    }
}

macro_rules! impl_di {
    ($($t:ty: $f:ident),+) => {$(
        impl FromRef<State> for $t {
            fn from_ref(state: &State) -> Self {
                state.$f.clone()
            }
        }
    )+};
}

impl_di!(BrowserService: browser);
impl_di!(TextExtractor: title);
