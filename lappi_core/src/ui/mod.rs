pub mod utils;

use amina_core::service::Context;

pub fn initialize(context: &Context) {
    context.init_service::<utils::UiUtils>();
}
