mod check;
mod parser;

use check::check;
use clap::Parser;
use parser::parse_txt;

enum Res {
    NoInternerConnection,
    WhiteListEnabled,
    FullInternetAvailable,
}

impl std::fmt::Display for Res {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Res::NoInternerConnection => write!(f, "Нет интернет соединения"),
            Res::WhiteListEnabled => write!(f, "Белые списки включены"),
            Res::FullInternetAvailable => write!(f, "Интернет не ограничен"),
        }
    }
}

#[derive(Parser)]
struct Args {

    /// Завершать проверку домена при успешном пинге
    #[arg(short, long)]
    successful_skip: bool,

    /// Количество попыток подключения к серверам
    #[arg(long, short, default_value = "3")]
    tries: u32,

    /// Путь к файлу с серверами в белом списке
    #[arg(long, short, default_value = "./wl_servers.txt")]
    whitelisted: String,

    /// Путь к файлу с серверами вне белого списка
    #[arg(long, short, default_value = "./nwl_servers.txt")]
    not_whitelisted: String,
}

fn check_urls(urls: Vec<String>, tries: u32) -> bool {
    let mut result: bool = true;
    for url in urls {
        for i in 0..tries {
            print!("Попытка {}/{} пинга {url}...", i + 1, tries);
            match check(url.clone()) {
                Ok(k) => match k {
                    Ok(rtt) => {
                        println!(" успех, отклик={rtt}мс");
                        break;
                    }
                    Err(e) => {
                        println!();
                        eprintln!("Ошибка: {e}");
                    }
                },
                Err((msg, err)) => {
                    println!();
                    eprintln!("{msg}: {err}");
                }
            }
            result = false;
        }
    }
    result
}

fn main() {
    let args = Args::parse();
    let mut result: Res = Res::NoInternerConnection;

    println!("Проверяем сервера из белого списка:");

    for url in parse_txt(args.whitelisted) {
        for i in 0..args.tries {
            if check(url.to_string(), i, args.tries) {
                result = Res::WhiteListEnabled;
                if args.successful_skip {
                    println!("Успех, переходим к следующему");
                    break;
                }
            } else {
                result = Res::NoInternerConnection;
            };
        }
    }

    println!("Проверяем сервера вне белого списка:");

    for url in parse_txt(args.not_whitelisted) {
        for i in 0..args.tries {
            if check(url.to_string(), i, args.tries) {
                result = Res::FullInternetAvailable;
                if args.successful_skip {
                    println!("Успех, переходим к следующему");
                    break;
                }
            }
        }
    }

    println!("===============Результат===============");
    println!("{result}");
    println!("=======================================");

    #[cfg(target_os = "windows")]
    {
        use std::io::stdin;

        let stdin = stdin();
        let mut s = "".to_string();

        println!("Нажмите Enter для выхода...");

        stdin.read_line(&mut s).expect("Ошибка чтения из stdin");
    }
}
