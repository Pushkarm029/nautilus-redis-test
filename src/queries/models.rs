use std::collections::HashMap;

pub trait CacheDatabaseAdapter {
    fn load(&self) -> anyhow::Result<HashMap<String, String>>;

    fn load_currencies(&mut self) -> anyhow::Result<std::sync::mpsc::Receiver<HashMap<String, String>>>;

    fn load_instruments(&mut self) -> anyhow::Result<std::sync::mpsc::Receiver<HashMap<String, String>>>;

    fn load_synthetics(&mut self) -> anyhow::Result<std::sync::mpsc::Receiver<HashMap<String, String>>>;

    fn load_accounts(&mut self) -> anyhow::Result<std::sync::mpsc::Receiver<HashMap<String, String>>>;

    fn load_orders(&mut self) -> anyhow::Result<std::sync::mpsc::Receiver<HashMap<String, String>>>;

    fn load_positions(&mut self) -> anyhow::Result<HashMap<String, String>>;
}
