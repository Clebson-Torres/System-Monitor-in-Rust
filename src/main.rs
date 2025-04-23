use sysinfo::{
    Components, Disks, Networks, System,
};
fn main() {

    let mut sys = System::new_all();

    sys.refresh_all();

    println!("=> system:");
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());
    loop {
        println!("\nChoose an option:");
        println!("1. Display system information");
        println!("2. Display disks information");
        println!("3. Display network information");
        println!("4. Display components temperature");
        println!("5. Exit");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("System name:             {:?}", System::name());
                println!("System kernel version:   {:?}", System::kernel_version());
                println!("System OS version:       {:?}", System::os_version());
                println!("System host name:        {:?}", System::host_name());
                println!("total memory: {} bytes", sys.total_memory());
                println!("used memory : {} bytes", sys.used_memory());
                println!("total swap  : {} bytes", sys.total_swap());
                println!("used swap   : {} bytes", sys.used_swap());
            }
            2 => {
                println!("=> disks:");
                let disks = Disks::new_with_refreshed_list();
                for disk in &disks {
                    println!("{disk:?}");
                }
            }
            3 => {
                let networks = Networks::new_with_refreshed_list();
                println!("=> networks:");
                for (interface_name, data) in &networks {
                    println!(
                        "{interface_name}: {} B (down) / {} B (up)",
                        data.total_received(),
                        data.total_transmitted(),
                    );
                }
            }
            4 => {
                let components = Components::new_with_refreshed_list();
                println!("=> components:");
                for component in &components {
                    println!("{component:?}");
                }
            }
            5 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option. Please choose a number between 1 and 5.");
            }
        }
    }
}