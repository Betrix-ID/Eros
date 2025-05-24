/* 
 notes critical user :
 - You may not change, modify or even put this script into your personal project without written permission from the script creator (official author) 
 
 Telegarm    : @UnixeID | Chenel : @Yeye_PID
 Github      : Betrix-ID
 Consultasi  : betrix322@gmail.com
 
 the date this script was written : 11 - Mei - 2025
*/

use std::{
     process::Command,
     io::{BufRead, BufReader},
     fs::{File, metadata},
     thread::sleep,
     time::Duration,
 };
     
fn delay(slow: u64) {
    sleep(Duration::from_secs(slow));
}

fn top_class(managers: &str) {
      if let Err(e) = Command::new("sh").arg("-c").arg(managers).status() {
         eprintln!("faoled running command {}", e);
      } else {
         println!("Ran: {}", managers);
   }
}
     
fn shell(message: &str) {
     let cmd = format!(
        "am start -a android.intent.action.MAIN -e toasttext \"{}\" \
        -n bellavita.toast/.MainActivity > /dev/null 2>&1",
        message
    );
  let _ = Command::new("sh").arg("-c").arg(cmd).status();
}
     
fn iorpot() {
    let cmd = [
        "cmd device_config put runtime_native_boot iorap_perfetto_enable true".to_string(),
        "cmd device_config put runtime_native_boot iorap_readahead_enable true".to_string(),
        "cmd device_config put runtime_native_boot enable_uffd_gc_2 true".to_string(),
        "cmd device_config put runtime_native_boot disable_lock_profiling true".to_string(),
        "cmd device_config put runtime_native_boot iorapd_options --readahead-mode=trigger_all".to_string(),
     ];
     
     for dcmd in cmd {
        let _ = Command::new("sh").arg("-c").arg(&dcmd).status();
     }
 } 
 
fn adjust_sysctl(pid: &str, high: bool) {
      let settings = if high {
            iorpot();
            format!("chrt -f -p 60 {}", pid)
           } else {
             format!("chrt -f -p 20 {}", pid)
      };
   let _ = Command::new("sh").arg("-c").arg(&settings).status();
}

fn priority_high() {
     let primarypath = "data/local/tmp/lnpng";    
        if metadata(primarypath).is_err() {
            println!("File not found !");
         return;
      }
      
     let input = Command::new("pgrep").arg("-f").arg(primarypath).output();
         let Ok(out) = input else {
            eprintln!("failed to run pgrep ");
        return;
      };
     
     let grep = String::from_utf8_lossy(&out.stdout).trim().to_string();
       if grep.is_empty() {
          println!("Application PID not found !");
          return;
      }
         
     let manager = [          
          format!("renice -n 0 -p {}", grep),
          format!("ionice -n 2 -c 0 -p {}", grep),
      ];
    
    for props in manager {
        top_class(&props);
     }     
 }
 
 
fn priority_loss() {
     let primarypath = "/data/local/tmp/lnpng";   
        if metadata(primarypath).is_err() {
            println!("File not found !");
         return;
      }
      
     let input = Command::new("pgrep").arg("-f").arg(primarypath).output();
         let Ok(out) = input else {
            eprintln!("failed to run pgrep ");
        return;
      };
      
     let grep = String::from_utf8_lossy(&out.stdout).trim().to_string();
       if grep.is_empty() {
          println!("Application PID not found !");
          return;
      }
      
     let manager = [          
          format!("renice -n 10 -p {}", grep),
          format!("iorenice {} 5 idle", grep),
      ];
    
    for props in manager {
        top_class(&props);
     }
 }
     
fn check_app_running(chracter_app: &str) -> Option<String> {
    let pidof = format!("pgrep -f {}", chracter_app);
    if let Ok(ore) = Command::new("sh").arg("-c").arg(&pidof).output() {
       if let Ok(result) = String::from_utf8(ore.stdout) {
          if let Some(pid_line) = result.lines().next() {
              return Some(pid_line.trim().to_string());
          }
      }
   }
     None
}
     
fn main() {
     let path = "/sdcard/eros/GamePid.txt";
     let mut pliable_name = String::new();
     
     shell("♨️ Multicor Priority is running in background");
     delay(1);
     shell("♨️ Multicor Priority: By @UnixeID");
     
     loop {
         let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                static mut ERROR_COUNT: usize =0;
                unsafe {
                     if ERROR_COUNT % 10 == 0 {
                         println!("File GamePid not found !");
                      }
                      ERROR_COUNT += 1;
                    }
                     delay(2);
                     continue;
                }
             };
                    
          let reader = BufReader::new(file);
          let mut app_found = false;
          
          for line in reader.lines() {
             if let Ok(app) = line {
                 if let Some(pid) = check_app_running(&app) {
                      app_found = true;
                      
               if app != pliable_name {
                  let pss = format!("♨️ Multicor Priority High : {}", app);
                  shell(&pss);
                  adjust_sysctl(&pid, true);
                  priority_high();
                  pliable_name = app.clone();            
               }
             
              break;
           }
       }
    }
      
      if !app_found && !pliable_name.is_empty() {
         let pss = "♨️ Multicor Priority Low : Game closs";
         shell(pss);
         if let Some(pid) = check_app_running(&pliable_name) {
            adjust_sysctl(&pid, false);
            priority_loss();
         }
          pliable_name.clear();
      }
      delay(2);
    }
}
