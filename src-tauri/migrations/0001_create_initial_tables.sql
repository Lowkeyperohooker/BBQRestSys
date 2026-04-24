CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS staff (
    staff_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    full_name TEXT NOT NULL,
    role TEXT NOT NULL,
    phone_number TEXT,
    passcode TEXT NOT NULL, 
    status TEXT DEFAULT 'Active',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS shift (
    shift_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    staff_id INTEGER NOT NULL,
    shift_date DATE NOT NULL,
    clock_in_time TIMESTAMP WITH TIME ZONE NOT NULL,
    clock_out_time TIMESTAMP WITH TIME ZONE,
    total_rendered_hours DECIMAL(5,2),
    status TEXT DEFAULT 'Active Shift',
    FOREIGN KEY (staff_id) REFERENCES staff(staff_id)
);

CREATE TABLE IF NOT EXISTS raw_inventory (
    raw_item_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    category TEXT NOT NULL,
    specific_part TEXT NOT NULL,
    current_stock_kg DECIMAL(10,2) DEFAULT 0.0,
    alert_threshold_kg DECIMAL(10,2) DEFAULT 0.0
);

CREATE TABLE IF NOT EXISTS prepared_inventory (
    prep_item_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    raw_item_id INTEGER,
    category TEXT NOT NULL DEFAULT 'Skewered', 
    pos_display_name TEXT NOT NULL,
    current_stock_pieces INTEGER DEFAULT 0,
    unit_price DECIMAL(10,2) NOT NULL,
    is_variable_price BOOLEAN DEFAULT FALSE,
    photo_url TEXT,
    FOREIGN KEY (raw_item_id) REFERENCES raw_inventory(raw_item_id)
);

CREATE TABLE IF NOT EXISTS prep_log (
    prep_log_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    staff_id INTEGER NOT NULL,
    raw_item_id INTEGER NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    kilos_deducted DECIMAL(10,2) NOT NULL,
    skewers_added INTEGER NOT NULL,
    FOREIGN KEY (staff_id) REFERENCES staff(staff_id),
    FOREIGN KEY (raw_item_id) REFERENCES raw_inventory(raw_item_id)
);

CREATE TABLE IF NOT EXISTS orders (
    order_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    staff_id INTEGER NOT NULL,
    customer_identifier TEXT NOT NULL,
    order_type TEXT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    subtotal DECIMAL(10,2) NOT NULL,
    tax_amount DECIMAL(10,2) NOT NULL,
    total_amount DECIMAL(10,2) NOT NULL,
    status TEXT DEFAULT 'Cooking',
    FOREIGN KEY (staff_id) REFERENCES staff(staff_id)
);

CREATE TABLE IF NOT EXISTS order_item (
    order_item_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    order_id INTEGER NOT NULL,
    prep_item_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    price_at_time_of_sale DECIMAL(10,2) NOT NULL,
    FOREIGN KEY (order_id) REFERENCES orders(order_id),
    FOREIGN KEY (prep_item_id) REFERENCES prepared_inventory(prep_item_id)
);

CREATE TABLE IF NOT EXISTS system_log (
    log_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    log_category TEXT NOT NULL,
    staff_id INTEGER NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    description TEXT NOT NULL,
    details TEXT,
    FOREIGN KEY (staff_id) REFERENCES staff(staff_id)
);

CREATE TABLE IF NOT EXISTS pos_category (
    category_name TEXT PRIMARY KEY,
    is_removable BOOLEAN DEFAULT TRUE
);

-- NEW: Database Sequence to auto-generate Kiosk numbers 1000 -> 9090
CREATE SEQUENCE IF NOT EXISTS kiosk_queue_seq START 1000 MAXVALUE 9090 CYCLE;

-- NEW: Tables for the pending Kiosk Queue
CREATE TABLE IF NOT EXISTS kiosk_queue (
    queue_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    queue_number INTEGER NOT NULL,
    order_type TEXT NOT NULL,
    subtotal DECIMAL(10,2) NOT NULL,
    tax DECIMAL(10,2) NOT NULL,
    total DECIMAL(10,2) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS kiosk_queue_item (
    queue_item_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    queue_id INTEGER NOT NULL REFERENCES kiosk_queue(queue_id) ON DELETE CASCADE,
    prep_item_id INTEGER NOT NULL REFERENCES prepared_inventory(prep_item_id),
    quantity INTEGER NOT NULL,
    unit_price DECIMAL(10,2) NOT NULL
);