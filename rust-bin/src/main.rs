use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "{{PROJECTNAME}}", about = "An example of StructOpt usage.")]
struct Opt {}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
