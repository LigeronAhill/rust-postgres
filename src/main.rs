use isac::AppResult;
use tracing::info;

fn main() -> AppResult<()> {
    let args = isac::get_cli();
    isac::logger::init(args.env(), args.level())?;
    info!("Запускается Интеллектуальный Системный Аналитический Комплекс");
    Ok(())
}
