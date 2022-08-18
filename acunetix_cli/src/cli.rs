use {argh::FromArgs, std::fmt::Debug};

#[derive(FromArgs, PartialEq, Debug)]
#[argh(description = "AcunetixCli")]
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
#[argh(subcommand, name = "target", description = "Target module.")]
struct SubCommandTarget {
    #[argh(positional, description = "target")]
    t: Vec<String>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "scan", description = "Scan module.")]
struct SubCommandScan {
    #[argh(switch, description = "or")]
    fooey: bool,
}