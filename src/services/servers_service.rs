use std::sync::Arc;

use crate::repositories::servers_repository::ServersRepository;

pub struct ServersService{
    servers_repository: Arc<dyn ServersRepository>,
}

impl ServersService{
    pub fn new(servers_repository: Arc<dyn ServersRepository>) -> Self{
        Self { servers_repository }
    }
    pub fn search_all_servers(&self){
        self.servers_repository.find_all();
    }
}
