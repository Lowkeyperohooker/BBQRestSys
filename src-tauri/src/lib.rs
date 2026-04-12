use tauri_plugin_sql::{Builder as SqlBuilder, Migration, MigrationKind};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn check_system() -> String {
    "BBQ Restaurant System is online and native backend is responding!".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Define the initial database schema
    let migrations = vec![Migration {
        version: 1,
        description: "create_initial_tables",
        sql: "
                CREATE TABLE IF NOT EXISTS Staff (
                    staff_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    full_name TEXT NOT NULL,
                    role TEXT NOT NULL,
                    phone_number TEXT,
                    status TEXT DEFAULT 'Active',
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                );

                CREATE TABLE IF NOT EXISTS Shift (
                    shift_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    staff_id INTEGER NOT NULL,
                    shift_date DATE NOT NULL,
                    clock_in_time DATETIME NOT NULL,
                    clock_out_time DATETIME,
                    total_rendered_hours DECIMAL(5,2),
                    status TEXT DEFAULT 'Active Shift',
                    FOREIGN KEY (staff_id) REFERENCES Staff(staff_id)
                );

                CREATE TABLE IF NOT EXISTS Raw_Inventory (
                    raw_item_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    category TEXT NOT NULL,
                    specific_part TEXT NOT NULL,
                    current_stock_kg DECIMAL(10,2) DEFAULT 0.0,
                    alert_threshold_kg DECIMAL(10,2) DEFAULT 0.0
                );

                CREATE TABLE IF NOT EXISTS Prepared_Inventory (
                    prep_item_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    raw_item_id INTEGER NOT NULL,
                    pos_display_name TEXT NOT NULL,
                    current_stock_pieces INTEGER DEFAULT 0,
                    unit_price DECIMAL(10,2) NOT NULL,
                    FOREIGN KEY (raw_item_id) REFERENCES Raw_Inventory(raw_item_id)
                );

                CREATE TABLE IF NOT EXISTS Prep_Log (
                    prep_log_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    staff_id INTEGER NOT NULL,
                    raw_item_id INTEGER NOT NULL,
                    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                    kilos_deducted DECIMAL(10,2) NOT NULL,
                    skewers_added INTEGER NOT NULL,
                    FOREIGN KEY (staff_id) REFERENCES Staff(staff_id),
                    FOREIGN KEY (raw_item_id) REFERENCES Raw_Inventory(raw_item_id)
                );

                CREATE TABLE IF NOT EXISTS Orders (
                    order_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    staff_id INTEGER NOT NULL,
                    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                    subtotal DECIMAL(10,2) NOT NULL,
                    tax_amount DECIMAL(10,2) NOT NULL,
                    total_amount DECIMAL(10,2) NOT NULL,
                    status TEXT DEFAULT 'Completed',
                    FOREIGN KEY (staff_id) REFERENCES Staff(staff_id)
                );

                CREATE TABLE IF NOT EXISTS Order_Item (
                    order_item_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    order_id INTEGER NOT NULL,
                    prep_item_id INTEGER NOT NULL,
                    quantity INTEGER NOT NULL,
                    price_at_time_of_sale DECIMAL(10,2) NOT NULL,
                    FOREIGN KEY (order_id) REFERENCES Orders(order_id),
                    FOREIGN KEY (prep_item_id) REFERENCES Prepared_Inventory(prep_item_id)
                );

                CREATE TABLE IF NOT EXISTS System_Log (
                    log_id TEXT PRIMARY KEY,
                    log_category TEXT NOT NULL,
                    staff_id INTEGER NOT NULL,
                    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                    description TEXT NOT NULL,
                    details TEXT,
                    FOREIGN KEY (staff_id) REFERENCES Staff(staff_id)
                );
            ",
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // Register the SQL plugin ONLY ONCE and execute migrations
        .plugin(
            SqlBuilder::default()
                .add_migrations("sqlite:bbq_system.db", migrations)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet, check_system])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}