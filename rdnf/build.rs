fn main(){
    cc::Build::new()
    .file("src/c_lib/queue_static.c")
    .file("src/c_lib/rpm_trans.c")
    .compile("queue_static");
}