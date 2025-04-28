use myio::read_line_with_prompt;

fn main() {
    let syear = read_line_with_prompt("syear: ");
    let sem = read_line_with_prompt("sem: ");
    let co_no = read_line_with_prompt("co_no: ");
    let class_code = read_line_with_prompt("class_code: ");

    println!(
        "https://qrys.ncku.edu.tw/ncku/histogram.asp?syear=0{}&sem={}&co_no={}&class_code={}\n",
        syear, sem, co_no, class_code,
    );
}
