#[cfg(windows)]
//extern create winres;
#[link(name = "winres")]

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico");
    res.compile().unwrap();
}