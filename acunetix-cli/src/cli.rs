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
#[argh(subcommand)]
enum SubCommandTarget {
    Mk(CreateTarget),
    Ls(ListTarget),
    Rm(RemoveTarget),
    Prune(PruneTarget),
}

#[derive(FromArgs, PartialEq, Debug)]
/// create target
#[argh(subcommand, name = "create")]
struct CreateTarget {
    #[argh(option)]
    /// how many x
    x: usize,
}

#[derive(FromArgs, PartialEq, Debug)]
/// list target
#[argh(subcommand, name = "list")]
struct ListTarget {
    #[argh(option)]
    /// how many x
    x: usize,
}

#[derive(FromArgs, PartialEq, Debug)]
/// remove target
#[argh(subcommand, name = "remove")]
struct RemoveTarget {
    #[argh(option)]
    /// how many x
    x: usize,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Prune target
#[argh(subcommand, name = "Prune")]
struct PruneTarget {
    #[argh(option)]
    /// how many x
    x: usize,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "scan", description = "Scan module.")]
struct SubCommandScan {
    #[argh(switch, description = "or")]
    fooey: bool,
}