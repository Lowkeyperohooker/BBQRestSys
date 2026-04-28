TRUNCATE TABLE 
    system_log, 
    order_item, 
    orders, 
    prep_log, 
    prepared_inventory, 
    raw_inventory, 
    shift, 
    staff, 
    pos_category 
RESTART IDENTITY CASCADE;

INSERT INTO pos_category (category_name, is_removable) VALUES
('Skewered', FALSE), ('Beverages', TRUE), ('Desserts', TRUE), ('Extras', TRUE);

INSERT INTO staff (full_name, role, phone_number, passcode, status) VALUES
('System Admin', 'Super Admin', '09171234567', '0000', 'Active'),
('John Doe', 'Admin', '09181234567', '1234', 'Active'),
('Jane Smith', 'Staff', '09191234567', '5678', 'Active'),
('Mark Inactive', 'Staff', '09201234567', '9999', 'Inactive');

INSERT INTO raw_inventory (category, specific_part, current_stock_kg, alert_threshold_kg) VALUES
('Pork', 'Pork Belly', 25.5, 5.0), 
('Pork', 'Intestines', 10.0, 3.0), 
('Pork', 'Ears', 8.5, 2.0),
('Chicken', 'Intestines', 15.0, 4.0), 
('Chicken', 'Breast', 20.0, 5.0), 
('Chicken', 'Feet', 5.0, 2.0),
('Chicken', 'Thigh', 12.0, 3.0);

INSERT INTO prepared_inventory (raw_item_id, category, pos_display_name, variant_group, variant_name, current_stock_pieces, unit_price) VALUES
((SELECT raw_item_id FROM raw_inventory WHERE category = 'Pork' AND specific_part = 'Pork Belly' LIMIT 1), 'Skewered', 'Pork BBQ', NULL, NULL, 150, 35.00),
((SELECT raw_item_id FROM raw_inventory WHERE category = 'Chicken' AND specific_part = 'Intestines' LIMIT 1), 'Skewered', 'Chicken Isaw', NULL, NULL, 200, 15.00),
((SELECT raw_item_id FROM raw_inventory WHERE category = 'Pork' AND specific_part = 'Intestines' LIMIT 1), 'Skewered', 'Pork Isaw', NULL, NULL, 180, 15.00),
((SELECT raw_item_id FROM raw_inventory WHERE category = 'Chicken' AND specific_part = 'Feet' LIMIT 1), 'Skewered', 'Adidas (Chicken Feet)', NULL, NULL, 60, 20.00),
((SELECT raw_item_id FROM raw_inventory WHERE category = 'Pork' AND specific_part = 'Ears' LIMIT 1), 'Skewered', 'Tenga (Pork Ears)', NULL, NULL, 45, 20.00),

((SELECT raw_item_id FROM raw_inventory WHERE category = 'Chicken' AND specific_part = 'Thigh' LIMIT 1), 'Skewered', 'Small Chicken Thigh', 'Chicken Thigh', 'Small', 30, 45.00),
((SELECT raw_item_id FROM raw_inventory WHERE category = 'Chicken' AND specific_part = 'Thigh' LIMIT 1), 'Skewered', 'Large Chicken Thigh', 'Chicken Thigh', 'Large', 20, 65.00),

(NULL, 'Beverages', 'Coke (8oz)', 'Soda (8oz)', 'Coke', 100, 20.00),
(NULL, 'Beverages', 'Sprite (8oz)', 'Soda (8oz)', 'Sprite', 50, 20.00),
(NULL, 'Beverages', 'Royal (8oz)', 'Soda (8oz)', 'Royal', 40, 20.00),

(NULL, 'Beverages', 'San Miguel Pale Pilsen', NULL, NULL, 48, 60.00),
(NULL, 'Desserts', 'Leche Flan', NULL, NULL, 20, 50.00),
(NULL, 'Extras', 'Extra Rice', NULL, NULL, 500, 20.00);

INSERT INTO shift (staff_id, shift_date, clock_in_time, clock_out_time, total_rendered_hours, status) VALUES
(2, CURRENT_DATE - INTERVAL '1 day', NOW() - INTERVAL '1 day 8 hours', NOW() - INTERVAL '1 day', 8.0, 'Completed'),
(3, CURRENT_DATE - INTERVAL '1 day', NOW() - INTERVAL '1 day 8.5 hours', NOW() - INTERVAL '1 day 0.5 hours', 8.0, 'Completed'),
(2, CURRENT_DATE, NOW() - INTERVAL '4 hours', NULL, NULL, 'Active Shift'),
(3, CURRENT_DATE, NOW() - INTERVAL '2 hours', NULL, NULL, 'Active Shift');

INSERT INTO prep_log (staff_id, raw_item_id, timestamp, kilos_deducted, skewers_added) VALUES
(3, (SELECT raw_item_id FROM raw_inventory WHERE specific_part = 'Pork Belly' LIMIT 1), NOW() - INTERVAL '3 hours', 5.0, 100),
(3, (SELECT raw_item_id FROM raw_inventory WHERE specific_part = 'Intestines' AND category = 'Chicken' LIMIT 1), NOW() - INTERVAL '2 hours', 2.0, 80);

INSERT INTO orders (staff_id, customer_identifier, order_type, timestamp, subtotal, tax_amount, total_amount, status) VALUES
(2, 'Queue #1001 - Table 4', 'Dine-in', NOW() - INTERVAL '1 hour', 160.00, 0.00, 160.00, 'Completed'),
(2, 'Queue #1002 - Table 12', 'Dine-in', NOW() - INTERVAL '10 minutes', 190.00, 0.00, 190.00, 'Cooking');

INSERT INTO order_item (order_id, prep_item_id, quantity, price_at_time_of_sale) VALUES
(1, (SELECT prep_item_id FROM prepared_inventory WHERE pos_display_name = 'Pork BBQ' LIMIT 1), 2, 35.00),
(1, (SELECT prep_item_id FROM prepared_inventory WHERE pos_display_name = 'Extra Rice' LIMIT 1), 2, 20.00),
(1, (SELECT prep_item_id FROM prepared_inventory WHERE pos_display_name = 'Coke (8oz)' LIMIT 1), 1, 20.00),
(1, (SELECT prep_item_id FROM prepared_inventory WHERE pos_display_name = 'Leche Flan' LIMIT 1), 1, 50.00),
(2, (SELECT prep_item_id FROM prepared_inventory WHERE pos_display_name = 'Chicken Isaw' LIMIT 1), 6, 15.00),
(2, (SELECT prep_item_id FROM prepared_inventory WHERE pos_display_name = 'Extra Rice' LIMIT 1), 2, 20.00),
(2, (SELECT prep_item_id FROM prepared_inventory WHERE pos_display_name = 'San Miguel Pale Pilsen' LIMIT 1), 1, 60.00);

INSERT INTO system_log (log_category, staff_id, timestamp, description, details) VALUES
('AUTH', 1, NOW() - INTERVAL '10 hours', 'System Boot', 'Database migrations verified and server started.'),
('INVENTORY', 2, NOW() - INTERVAL '5 hours', 'Stock Delivery Added', 'Item Restocked: Pork - Pork Belly\nAmount Added: 10.00 kg'),
('TIME', 2, NOW() - INTERVAL '4 hours', 'Clocked In', NULL),
('TIME', 3, NOW() - INTERVAL '2 hours', 'Clocked In', NULL),
('POS', 2, NOW() - INTERVAL '1 hour', 'Payment Settled', 'Order #1 has been paid in full.');