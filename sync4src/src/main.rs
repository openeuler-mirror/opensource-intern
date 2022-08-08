mod conf;
mod sync;

fn main() {
    let conf = conf::parse_conf().expect("failed to parse config");

    let repo = sync::clone(&conf.source).expect("failed to clone repository");

    sync::push_to_target(&repo, &conf.target).expect("failed to push to target repository");
}
