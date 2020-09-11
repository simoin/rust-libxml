fn main() {
  if pkg_config::find_library("libxml-2.0").is_ok() {
  } else if vcpkg::find_package("libxml2").is_ok() {
  } else {
    panic!("Could not find libxml2")
  }
}
