// @generated automatically by Diesel CLI.

diesel::table! {
    crawler (id) {
        id -> Int4,
        ip_id -> Int4,
        endpoint_id -> Int4,
    }
}

diesel::table! {
    endpoint (id) {
        id -> Int4,
        #[max_length = 255]
        route -> Nullable<Varchar>,
        status_code -> Nullable<Int4>,
        credentials -> Nullable<Bool>,
        authentification -> Nullable<Bool>,
        port_id -> Int4,
        ip_id -> Int4,
    }
}

diesel::table! {
    geo_metrics (id) {
        id -> Int4,
        ip_id -> Int4,
        #[max_length = 255]
        city -> Varchar,
        #[max_length = 255]
        hops_to_turkey -> Nullable<Varchar>,
        #[max_length = 255]
        hops_to_iraq -> Nullable<Varchar>,
        #[max_length = 255]
        hops_to_jordan -> Nullable<Varchar>,
        #[max_length = 255]
        hops_to_lebanon -> Nullable<Varchar>,
        #[max_length = 255]
        hops_to_israel -> Nullable<Varchar>,
    }
}

diesel::table! {
    ip (id) {
        id -> Int4,
        subnet_id -> Int4,
        #[max_length = 255]
        v4 -> Nullable<Varchar>,
        #[max_length = 255]
        v6 -> Nullable<Varchar>,
    }
}

diesel::table! {
    network_metrics (id) {
        id -> Int4,
        ip_id -> Int4,
        pingable -> Nullable<Bool>,
        reachable -> Nullable<Bool>,
        unreachable -> Nullable<Bool>,
        ttr -> Nullable<Float8>,
        ttl -> Nullable<Float8>,
    }
}

diesel::table! {
    port (id) {
        id -> Int4,
        ip_id -> Int4,
        number -> Int4,
        #[max_length = 255]
        service -> Nullable<Varchar>,
        timeout -> Nullable<Bool>,
        answer -> Nullable<Bool>,
        deny -> Nullable<Bool>,
    }
}

diesel::table! {
    subnet (id) {
        id -> Int4,
        target_id -> Int4,
        #[max_length = 255]
        cidr -> Varchar,
    }
}

diesel::table! {
    target (id) {
        id -> Int4,
        #[max_length = 255]
        country -> Varchar,
    }
}

diesel::table! {
    webserver (id) {
        id -> Int4,
        ip_id -> Int4,
        port_id -> Int4,
        crawer_id -> Int4,
        reachable -> Nullable<Bool>,
        #[max_length = 255]
        framework -> Nullable<Varchar>,
    }
}

diesel::joinable!(crawler -> endpoint (endpoint_id));
diesel::joinable!(crawler -> ip (ip_id));
diesel::joinable!(endpoint -> ip (ip_id));
diesel::joinable!(endpoint -> port (port_id));
diesel::joinable!(geo_metrics -> ip (ip_id));
diesel::joinable!(ip -> subnet (subnet_id));
diesel::joinable!(network_metrics -> ip (ip_id));
diesel::joinable!(port -> ip (ip_id));
diesel::joinable!(subnet -> target (target_id));
diesel::joinable!(webserver -> crawler (crawer_id));
diesel::joinable!(webserver -> ip (ip_id));
diesel::joinable!(webserver -> port (port_id));

diesel::allow_tables_to_appear_in_same_query!(
    crawler,
    endpoint,
    geo_metrics,
    ip,
    network_metrics,
    port,
    subnet,
    target,
    webserver,
);
