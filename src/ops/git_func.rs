pub fn parse_args(args: &String) -> Vec<&str> {
    args.split(" ").collect()
}