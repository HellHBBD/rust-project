use std::io::{self, Write};

// pub fn _read_line_with_prompt<R: Read + BufRead, W: Write>(
//     prompt: &str,
//     reader: &mut R,
//     writer: &mut W,
// ) -> io::Result<String> {
//     writer.write_all(prompt.as_bytes())?;
//     writer.flush()?;
//
//     let mut line = String::new();
//     reader.read_line(&mut line)?;
//     Ok(line.trim().to_owned())
// }
//
// pub fn read_line_with_prompt(prompt: &str) -> String {
//     _read_line_with_prompt(prompt, &mut io::stdin(), &mut io::stdout()).expect("fail to read line")
// }

pub fn read_line_with_prompt(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_owned()
}
