mod error;
mod interface;
mod mock;
mod DbCommand;

pub use error::{KvsError,KvsResult};
pub use mock::MockConnector;
pub use interface::{Communicate,Connect};
