fn main () {
  println!("cargo:rustc-env=DISCORD_CLIENT_ID={}", env!("DISCORD_CLIENT_ID"));
}