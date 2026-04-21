CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS Staff (
    staff_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    full_name TEXT NOT NULL,
    role TEXT NOT NULL,
    phone_number TEXT,
    passcode TEXT NOT NULL, -- NEW: Required for Login
    status TEXT DEFAULT 'Active',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Shift (
    shift_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    staff_id INTEGER NOT NULL,
    shift_date DATE NOT NULL,
    clock_in_time TIMESTAMP WITH TIME ZONE NOT NULL,
    clock_out_time TIMESTAMP WITH TIME ZONE,
    total_rendered_hours DECIMAL(5,2),
    status TEXT DEFAULT 'Active Shift',
    FOREIGN KEY (staff_id) REFERENCES Staff(staff_id)
);

CREATE TABLE IF NOT EXISTS Raw_Inventory (
    raw_item_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    category TEXT NOT NULL,
    specific_part TEXT NOT NULL,
    current_stock_kg DECIMAL(10,2) DEFAULT 0.0,
    alert_threshold_kg DECIMAL(10,2) DEFAULT 0.0
);

CREATE TABLE IF NOT EXISTS Prepared_Inventory (
    prep_item_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    raw_item_id INTEGER NOT NULL,
    pos_display_name TEXT NOT NULL,
    current_stock_pieces INTEGER DEFAULT 0,
    unit_price DECIMAL(10,2) NOT NULL,
    is_variable_price BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (raw_item_id) REFERENCES Raw_Inventory(raw_item_id)
);

CREATE TABLE IF NOT EXISTS Prep_Log (
    prep_log_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    staff_id INTEGER NOT NULL,
    raw_item_id INTEGER NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    kilos_deducted DECIMAL(10,2) NOT NULL,
    skewers_added INTEGER NOT NULL,
    FOREIGN KEY (staff_id) REFERENCES Staff(staff_id),
    FOREIGN KEY (raw_item_id) REFERENCES Raw_Inventory(raw_item_id)
);

CREATE TABLE IF NOT EXISTS Orders (
    order_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    staff_id INTEGER NOT NULL,
    customer_identifier TEXT NOT NULL,
    order_type TEXT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    subtotal DECIMAL(10,2) NOT NULL,
    tax_amount DECIMAL(10,2) NOT NULL,
    total_amount DECIMAL(10,2) NOT NULL,
    status TEXT DEFAULT 'Cooking',
    FOREIGN KEY (staff_id) REFERENCES Staff(staff_id)
);

CREATE TABLE IF NOT EXISTS Order_Item (
    order_item_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    order_id INTEGER NOT NULL,
    prep_item_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    price_at_time_of_sale DECIMAL(10,2) NOT NULL,
    FOREIGN KEY (order_id) REFERENCES Orders(order_id),
    FOREIGN KEY (prep_item_id) REFERENCES Prepared_Inventory(prep_item_id)
);

CREATE TABLE IF NOT EXISTS System_Log (
    log_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    log_category TEXT NOT NULL,
    staff_id INTEGER NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    description TEXT NOT NULL,
    details TEXT,
    FOREIGN KEY (staff_id) REFERENCES Staff(staff_id)
);