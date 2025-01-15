use crate::AppResult;

pub fn init(env: crate::Environment, level: tracing::Level) -> AppResult<()> {
    match env {
        crate::Environment::Dev => {
            let subscriber = tracing_subscriber::FmtSubscriber::builder()
                .with_max_level(level)
                .with_file(true)
                .with_line_number(true)
                .with_target(false)
                .pretty()
                .finish();
            tracing::subscriber::set_global_default(subscriber)?;
        }
        crate::Environment::Prod => {
            let file =
                std::fs::File::create("isac.json").map_err(|_| crate::AppError::FileError)?;
            let subscriber = tracing_subscriber::FmtSubscriber::builder()
                .with_max_level(level)
                .with_file(true)
                .with_line_number(true)
                .with_target(false)
                .json()
                .with_writer(file)
                .finish();
            tracing::subscriber::set_global_default(subscriber)?;
        }
    }
    tracing::info!("Информационные сообщения включены");
    tracing::warn!("Предупреждающие сообщения включены");
    tracing::error!("Сообщения об ошибках включены");
    tracing::debug!("Отладочные сообщения включены");
    tracing::trace!("Сообщения типа trace включены");
    Ok(())
}
