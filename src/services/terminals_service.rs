use uuid::Uuid;

async fn create_if_not_exists(terminal_id: Uuid) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!("no terminal"))
}
