use crate::states::repositories::Repositories;

pub struct ServiceState {
    pub repositories: Repositories,
}

impl ServiceState {
    pub fn new(repositories: Repositories) -> Self {
        Self { repositories }
    }
}
