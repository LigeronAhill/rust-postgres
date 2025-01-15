use isac::AppResult;
use tracing::info;

fn main() -> AppResult<()> {
    let args = isac::get_cli();
    isac::logger::init(args.env(), args.level())?;
    info!("Запускается Интеллектуальный Системный Аналитический Комплекс");
    let config = isac::configuration::init(args.config())?;
    for (k, v) in config {
        info!("{k}: {v:?}")
    }
    Ok(())
}
