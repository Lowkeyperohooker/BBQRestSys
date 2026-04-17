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
    // Define the database schema and starter data
    let migrations = vec![
        Migration {
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
                    is_variable_price BOOLEAN DEFAULT 0, -- NEW: 0 = Fixed Price, 1 = Ask Cashier
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
                    customer_identifier TEXT NOT NULL, -- NEW: e.g., 'Table 4' or 'Takeout - Juan'
                    order_type TEXT NOT NULL,          -- NEW: 'Dine-in' or 'Takeout'
                    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                    subtotal DECIMAL(10,2) NOT NULL,
                    tax_amount DECIMAL(10,2) NOT NULL,
                    total_amount DECIMAL(10,2) NOT NULL,
                    status TEXT DEFAULT 'Cooking',     -- CHANGED: Defaults to Cooking instead of Completed
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
        },
        Migration {
            version: 2,
            description: "insert_starter_data",
            sql: "
                INSERT INTO Staff (full_name, role, phone_number, status) VALUES
                ('Juan Dela Cruz', 'Grill Cook', '09123456789', 'Active'),
                ('Jane Smith', 'Prep Station', '555-0101', 'Active');

                INSERT INTO Raw_Inventory (category, specific_part, current_stock_kg, alert_threshold_kg) VALUES
                ('Chicken', 'Intestine', 14.5, 5.0),
                ('Chicken', 'Neck', 18.0, 8.0),
                ('Chicken', 'Leg Quarter', 42.5, 15.0),
                ('Chicken', 'Liver', 2.1, 5.0),
                ('Pork', 'Intestine', 22.0, 10.0),
                ('Pork', 'Liver', 4.5, 8.0),
                ('Seafood', 'Milk Fish Fillet', 15.0, 5.0),
                ('Seafood', 'Tuna Fillet', 18.5, 5.0);

                INSERT INTO Prepared_Inventory (raw_item_id, pos_display_name, current_stock_pieces, unit_price, is_variable_price) VALUES
                (1, 'Chicken Isaw', 145, 15.00, 0),        
                (2, 'Chicken Neck', 80, 20.00, 1),          
                (3, 'Chicken Leg Quarter', 120, 120.00, 1), 
                (4, 'Chicken Liver', 12, 20.00, 0),         
                (5, 'Pork Isaw', 205, 15.00, 0),            
                (6, 'Pork Liver', 35, 20.00, 0),            
                (7, 'Bangus Fillet', 85, 150.00, 1),        
                (8, 'Tuna Fillet', 90, 200.00, 1);          

                -- ==========================================
                -- MOCK HISTORICAL DATA
                -- ==========================================

                -- 1. Mock Historical Orders (Last 7 Days) for Sales Chart
                INSERT INTO Orders (staff_id, customer_identifier, order_type, timestamp, subtotal, tax_amount, total_amount, status) VALUES
                (1, 'Table 1', 'Dine-in', datetime('now'), 850.00, 68.00, 918.00, 'Completed'),
                (2, 'Takeout - Ana', 'Takeout', datetime('now', '-1 day'), 1200.00, 96.00, 1296.00, 'Completed'),
                (1, 'Table 4', 'Dine-in', datetime('now', '-2 days'), 1850.00, 148.00, 1998.00, 'Completed');

                -- 2. Mock Order Items
                INSERT INTO Order_Item (order_id, prep_item_id, quantity, price_at_time_of_sale) VALUES
                (1, 1, 10, 15.00), (1, 3, 5, 140.00),
                (2, 5, 20, 15.00), (2, 7, 6, 150.00),
                (3, 1, 30, 15.00), (3, 8, 7, 200.00);

                -- 3. Mock Prep Logs
                INSERT INTO Prep_Log (staff_id, raw_item_id, timestamp, kilos_deducted, skewers_added) VALUES
                (2, 1, datetime('now', '-1 day'), 25.5, 255), -- Chicken
                (2, 3, datetime('now', '-2 days'), 40.0, 120); -- Chicken

                -- 4. NEW: Mock Detailed System Logs to match the detailed format
                INSERT INTO System_Log (log_id, log_category, staff_id, timestamp, description, details) VALUES
                (hex(randomblob(8)), 'POS', 1, datetime('now'), 'Order Sent to Grill', 'Order Type: Dine-in
Customer/Table: Table 1

Items Ordered:
- 10x Chicken Isaw (₱150.00)
- 5x Chicken Leg Quarter (₱700.00)

Total Amount: ₱918.00'),

                (hex(randomblob(8)), 'PREP', 2, datetime('now', '-1 day'), 'Skewers Prepared', 'Meat Category: Chicken
Specific Part / Cut: Intestine
Raw Meat Consumed: 25.50 kg
Yield Produced: 255 pieces/sticks
Staff Member: Jane Smith'),

                (hex(randomblob(8)), 'POS', 2, datetime('now', '-1 day'), 'Order Sent to Grill', 'Order Type: Takeout
Customer/Table: Takeout - Ana

Items Ordered:
- 20x Pork Isaw (₱300.00)
- 6x Bangus Fillet (₱900.00)

Total Amount: ₱1296.00');
            ",
            kind: MigrationKind::Up,
        }
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            SqlBuilder::default()
                .add_migrations("sqlite:bbq_system.db", migrations)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet, check_system])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}