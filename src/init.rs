use crate::utils::readln_trimmed;
use std::{env::consts::OS, fs::File, path::PathBuf, process::Command};

const INIT_MODE_MSG: &str = "\n
enter mode:
1 - scale
2 - convert (еще нет)
0, exit - exit";

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    println!("{INIT_MODE_MSG}");
    let mode: isize = readln_trimmed().parse()?;
    if mode == 0 {
        std::process::exit(0);
    }
    println!("Путь к папке:");
    let path: String = readln_trimmed()
        .trim_matches(|c| c == '\"' || c == '\'')
        .to_string();
    //std::fs::File::open(&path)?; //на винде не работает, нет доступа
    let result = match mode {
        1 => mode_scale(path),
        0 => Err("exit")?,
        _ => Err("wrong mode")?,
    };
    if result.is_err() {
        println!("{}", result.unwrap_err());
    }

    Ok(())
}

//TODO: раскидать по функциям
//
//fn get_vf_filter -> String
//fn get_input_path
//fn get_output_path
fn mode_scale(path: String) -> Result<(), Box<dyn std::error::Error>> {

    println!("\nВведите ширину в пикселях. \n0 - оставить как есть, \n-1 - подстроить под высоту");
    let width: isize = readln_trimmed().parse()?;

    println!("\nВведите высоту в пикселях. \n0 - оставить как есть, \n-1 - подстроить под ширину");
    let height: isize = readln_trimmed().parse()?;

    let mut output_dir = PathBuf::from(&path);
    output_dir.push("converted");

    _ = std::fs::create_dir(&output_dir);
    println!("test");
        //File::open(&output_dir)?; // проверка, создалась ли папка. на винде не работает - нет доступа для проверки
    println!("\nПапка converted создана\n\n");

    let file_list = std::fs::read_dir(&path).expect("no directory");

    let ffmpeg_command = if OS == "linux" && File::open("./ffmpeg").is_ok() {
        "./ffmpeg"
    } else {
        "ffmpeg"
    };
    check_command(ffmpeg_command.to_string());

    for input_file in file_list {
        if input_file.is_err() {
            continue;
        }

        let input_file = input_file.unwrap();
        if input_file.file_type().is_ok_and(|file| file.is_dir()) {
            continue;
        }

        let mut output_path = output_dir.clone();
        output_path.push(input_file.file_name());

        //весь запуск тоже вынести в функцию
        let mut ffmpeg_exec = std::process::Command::new(ffmpeg_command);
        ffmpeg_exec
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
        ffmpeg_exec.arg("-n");
        ffmpeg_exec.arg("-i");
        ffmpeg_exec.arg(input_file.path());
        ffmpeg_exec.arg("-vf");
        let vf_filter = String::from("scale=") + &width.to_string() + ":" + &height.to_string();
        ffmpeg_exec.arg(vf_filter);
        ffmpeg_exec.arg(output_path);

        let status = ffmpeg_exec.status()?;

        if status.success() {
            println!("готово: {}", input_file.file_name().to_string_lossy());
        } else {
            println!(
                "не сконвертировано: {}",
                input_file.file_name().to_string_lossy()
            );
        }
    }
    Ok(())
}

fn check_command(command: String) {
    let execute = Command::new(command)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
    if execute.is_err() {
        println!("\nERROR: can't execute ffmpeg\n");
        std::process::exit(1);
    }
    execute
        .unwrap()
        .kill()
        .expect("\n\nProgram doesn't respond\n\n");
}
