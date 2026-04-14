erDiagram
    Staff {
        int staff_id PK
        varchar full_name
        varchar role "Enum: Cashier, Prep Station, Grill Cook, Manager"
        varchar phone_number
        varchar status "Enum: Active, Inactive"
        timestamp created_at
    }

    Shift {
        int shift_id PK
        int staff_id FK
        date shift_date
        datetime clock_in_time
        datetime clock_out_time
        decimal total_rendered_hours
        varchar status "Enum: Active Shift, Completed"
    }

    Raw_Inventory {
        int raw_item_id PK
        varchar category "Enum: Chicken, Pork, Seafood"
        varchar specific_part
        decimal current_stock_kg
        decimal alert_threshold_kg
    }

    Prepared_Inventory {
        int prep_item_id PK
        int raw_item_id FK
        varchar pos_display_name
        int current_stock_pieces
        decimal unit_price
    }

    Prep_Log {
        int prep_log_id PK
        int staff_id FK
        int raw_item_id FK
        datetime timestamp
        decimal kilos_deducted
        int skewers_added
    }

    Order {
        int order_id PK "e.g., Ticket #2045"
        int staff_id FK
        datetime timestamp
        decimal subtotal
        decimal tax_amount
        decimal total_amount
        varchar status "Enum: Completed, Refunded"
    }

    Order_Item {
        int order_item_id PK
        int order_id FK
        int prep_item_id FK
        int quantity
        decimal price_at_time_of_sale
    }

    System_Log {
        varchar log_id PK "e.g., POS-2045, PRP-1045"
        varchar log_category "Enum: POS, Prep, Inventory"
        int staff_id FK
        datetime timestamp
        varchar description
        text details
    }

    %% Relationships
    Staff ||--o{ Shift : "works"
    Staff ||--o{ Prep_Log : "performs"
    Staff ||--o{ Order : "processes"
    Staff ||--o{ System_Log : "generates"
    
    Raw_Inventory ||--o{ Prepared_Inventory : "processed_into"
    Raw_Inventory ||--o{ Prep_Log : "depleted_by"
    
    Order ||--|{ Order_Item : "contains"
    Prepared_Inventory ||--o{ Order_Item : "sold_as"