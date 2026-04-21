INSERT INTO Staff (full_name, role, phone_number, status, passcode) VALUES
('Juan Dela Cruz', 'Admin', '09123456789', 'Active', '1234'),
('Jane Smith', 'Staff', '123456789', 'Active', '5678'),
('System Developer', 'Super Admin', '000-0000', 'Active', '9999');

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
(1, 'Chicken Isaw', 145, 6.00, FALSE),
(2, 'Chicken Neck', 80, 5.00, TRUE),
(3, 'Chicken Leg Quarter', 120, 80.00, TRUE),
(4, 'Chicken Liver', 12, 15.00, FALSE),
(5, 'Pork Isaw', 205, 15.00, FALSE),
(6, 'Pork Liver', 35, 20.00, FALSE),
(7, 'Bangus Fillet', 85, 100.00, TRUE),
(8, 'Tuna Fillet', 90, 100.00, TRUE);

INSERT INTO Orders (staff_id, customer_identifier, order_type, timestamp, subtotal, tax_amount, total_amount, status) VALUES
(1, 'Table 1',      'Dine-in', NOW(),                       850.00,  68.00,  918.00,  'Completed'),
(2, 'Takeout - Ana','Takeout', NOW() - INTERVAL '1 day',   1200.00, 96.00,  1296.00, 'Completed'),
(1, 'Table 4',      'Dine-in', NOW() - INTERVAL '2 days',  1850.00, 148.00, 1998.00, 'Completed');

INSERT INTO Order_Item (order_id, prep_item_id, quantity, price_at_time_of_sale) VALUES
(1, 1, 10, 15.00), (1, 3, 5,  140.00),
(2, 5, 20, 15.00), (2, 7, 6,  150.00),
(3, 1, 30, 15.00), (3, 8, 7,  200.00);

INSERT INTO Prep_Log (staff_id, raw_item_id, timestamp, kilos_deducted, skewers_added) VALUES
(2, 1, NOW() - INTERVAL '1 day',  25.5, 255),
(2, 3, NOW() - INTERVAL '2 days', 40.0, 120);

INSERT INTO System_Log (log_category, staff_id, timestamp, description, details) VALUES
('POS', 1, NOW(), 'Order Sent to Grill',
'Order Type: Dine-in
Customer/Table: Table 1

Items Ordered:
- 10x Chicken Isaw (₱150.00)
- 5x Chicken Leg Quarter (₱700.00)

Total Amount: ₱918.00'),

('PREP', 2, NOW() - INTERVAL '1 day', 'Skewers Prepared',
'Meat Category: Chicken
Specific Part / Cut: Intestine
Raw Meat Consumed: 25.50 kg
Yield Produced: 255 pieces/sticks
Staff Member: Jane Smith'),

('POS', 2, NOW() - INTERVAL '1 day', 'Order Sent to Grill',
'Order Type: Takeout
Customer/Table: Takeout - Ana

Items Ordered:
- 20x Pork Isaw (₱300.00)
- 6x Bangus Fillet (₱900.00)

Total Amount: ₱1296.00');