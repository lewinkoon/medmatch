use log::error;
use log::info;
use medmatch::*;
use std::process;

fn main() {
    // initialize logger
    env_logger::init();

    // define file paths
    let applicants_path: &str = "files/applicants.csv";
    let vacancies_path: &str = "files/vacancies.csv";
    let preferences_path: &str = "files/preferences.csv";
    let board_path: &str = "results/board.csv";

    // read applicants requests file
    let applicants: Vec<Applicant> = parse_file(applicants_path).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    info!("Loaded {} applicant requests.", applicants.len());

    // read vacant positions file
    let vacancies: Vec<Vacancy> = parse_file(vacancies_path).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    info!("Loaded {} vacancy positions.", vacancies.len());

    // build requests board
    let mut requests: Vec<Request> = Vec::new();
    let mut rdr = csv::Reader::from_path(preferences_path).unwrap();
    for row in rdr.records() {
        let record: Request = Request::build(row.unwrap(), applicants.as_ref(), vacancies.as_ref());
        requests.push(record);
    }
    info!("Loaded {} requests.", requests.len());

    // write requests board to file
    if let Err(e) = export(board_path, requests) {
        error!("{}", e);
        process::exit(1)
    }
    info!("Requests board successfully written.")
}
