use {argh::FromArgs, std::fmt::Debug};

#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command.
pub struct TopLevel {
    #[argh(subcommand)]
    nested: MySubCommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum MySubCommandEnum {
    Target(SubCommandTarget),
    Scan(SubCommandScan),
}

#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "target")]
struct SubCommandTarget {
    #[argh(option)]
    /// how many x
    x: usize,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Second subcommand.
#[argh(subcommand, name = "scan")]
struct SubCommandScan {
    #[argh(switch)]
    /// whether to fooey
    fooey: bool,
}