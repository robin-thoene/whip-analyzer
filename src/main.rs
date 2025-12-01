use whip_analyzer::{
    args::get_cli_args, logger::setup_logger, parser::parse_cd_rip_results, printer::print_results,
};

fn main() {
    let args = get_cli_args();
    setup_logger();
    let cd_results = parse_cd_rip_results(&args.path);
    print_results(cd_results);
}
