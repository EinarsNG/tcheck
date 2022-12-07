use std::io::{stdin, BufRead, Error};
use argparse::{StoreTrue, ArgumentParser};

mod process;
use process::get_image_transparent;

fn main()
{
    let mut reverse = false;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut reverse)
            .add_option(&["-o", "--opaque"], StoreTrue,
            "Print opaque image paths instead");
        ap.parse_args_or_exit();
    }

    let process_image = |line: Result<String, Error>| {
        let path = line.as_ref().expect("Failed to read line").trim();
        if let Some(res) = get_image_transparent(path)
        {
            if res && !reverse
            {
                println!("{}", path);
            }
            else if !res && reverse
            {
                println!("{}", path);
            }
        }
        else
        {
            eprintln!("Failed to open image file: {}", path);
        }
    };

    stdin().lock().lines().for_each(process_image);
}
