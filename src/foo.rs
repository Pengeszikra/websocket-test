#[allow(dead_code)]
 pub fn hot_reload () {
   println!("
   ------[ rust hot reloading ]----
   
   cargo watch -x 'run'
 
   :: quiet :: -q
   
   cargo watch -x 'run' -q
 
   :: clear :: -c
 
   cargo watch -x 'run' -q -c
 
   :: delay :: -d default: 0.5 (s)
 
   cargo watch -x 'run' -q -c -d 1
   ");
}
