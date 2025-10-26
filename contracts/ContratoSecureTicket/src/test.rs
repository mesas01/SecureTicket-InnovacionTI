#![cfg(test)]
extern crate std;

use crate::{SecureTicketContract, SecureTicketContractClient};
use soroban_sdk::{
    testutils::{Address as _, Ledger}, // Se eliminó BytesN
    token::Client as TokenClient,
    token::StellarAssetClient,
    Address, Env, IntoVal,
};

// --- Helper: Configuración del Entorno Básico ---
fn setup_env<'a>() -> (
    Env,
    SecureTicketContractClient<'a>,
    Address, // Admin (para el token)
    Address, // Organizer (Tiquetera)
    Address, // Platform (Tu empresa)
    TokenClient<'a>, // Cliente de Token
) {
    let env = Env::default();
    env.mock_all_auths();

    // Cuentas
    let admin = Address::generate(&env);
    let organizer = Address::generate(&env);
    let platform = Address::generate(&env);

    // Contrato principal
    let contract_id = env.register(SecureTicketContract, ());
    let client = SecureTicketContractClient::new(&env, &contract_id);

    // Contrato de Token
    let token_contract = env.register_stellar_asset_contract_v2(admin.clone());
    let token_client = TokenClient::new(&env, &token_contract.address());

    // Inicializar el contrato
    client.initialize(
        &organizer,
        &platform,
        &token_contract.address(),
        &20_i128, // 20% comisión para organizador
        &10_i128, // 10% comisión para plataforma
    );

    (env, client, admin, organizer, platform, token_client)
}

// --- Helper: Creación de Fondos ---
fn mint_tokens(
    env: &Env,
    _token_admin: &Address,
    token_client: &TokenClient,
    user: &Address,
    amount: i128,
) {
    let stellar_asset_client = StellarAssetClient::new(env, &token_client.address);
    stellar_asset_client.mint(user, &amount); // Corregido: añadir &
}

// --- Helper: Crear y Listar un Ticket Primario ---
// CORREGIDO: Acepta un ticket_id para evitar conflictos
fn create_and_list_primary_ticket(
    client: &SecureTicketContractClient,
    ticket_id: u32,
    event_id: u32,
    price: i128,
) {
    client.create_ticket(&event_id, &price); // Crea el ticket (su ID será ticket_id)
    client.list_ticket(&ticket_id, &price); // Lista el ticket
}

// --- Helper: Vender un Ticket Primario ---
// CORREGIDO: Acepta un ticket_id para evitar conflictos
fn sell_primary_ticket<'a>(
    env: &Env,
    client: &SecureTicketContractClient,
    token_admin: &Address,
    token_client: &TokenClient,
    buyer: &Address,
    ticket_id: u32,
    event_id: u32,
    price: i128,
) {
    create_and_list_primary_ticket(client, ticket_id, event_id, price);
    mint_tokens(env, token_admin, token_client, buyer, price);
    client.buy_ticket(&ticket_id, buyer);
}

// --- Pruebas de Inicialización ---

#[test]
fn test_initialize_success() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let organizer = Address::generate(&env);
    let platform = Address::generate(&env);
    let contract_id = env.register(SecureTicketContract, ());
    let client = SecureTicketContractClient::new(&env, &contract_id);
    let token_contract = env.register_stellar_asset_contract_v2(admin.clone());

    client.initialize(&organizer, &platform, &token_contract.address(), &15_i128, &10_i128);
}

#[test]
#[should_panic(expected = "already_init")]
fn test_initialize_twice_panics() {
    let (_env, client, _admin, organizer, platform, token_client) = setup_env();
    client.initialize(
        &organizer,
        &platform,
        &token_client.address,
        &10_i128,
        &5_i128,
    );
}

#[test]
#[should_panic(expected = "fees_too_high")]
fn test_initialize_fees_too_high_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let organizer = Address::generate(&env);
    let platform = Address::generate(&env);
    let contract_id = env.register(SecureTicketContract, ());
    let client = SecureTicketContractClient::new(&env, &contract_id);
    let token_contract = env.register_stellar_asset_contract_v2(admin.clone());

    client.initialize(&organizer, &platform, &token_contract.address(), &80_i128, &20_i128);
}

#[test]
#[should_panic(expected = "negative_fees")]
fn test_initialize_negative_fees_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let organizer = Address::generate(&env);
    let platform = Address::generate(&env);
    let contract_id = env.register(SecureTicketContract, ());
    let client = SecureTicketContractClient::new(&env, &contract_id);
    let token_contract = env.register_stellar_asset_contract_v2(admin.clone());

    client.initialize(&organizer, &platform, &token_contract.address(), &-5_i128, &10_i128);
}

// --- Pruebas de Creación de Ticket ---

#[test]
fn test_create_ticket_success() {
    let (_env, client, _admin, organizer, _, _) = setup_env();

    client.create_ticket(&1001, &1_000_000);
    let ticket = client.get_ticket(&0);
    assert_eq!(ticket.id, 0);
    assert_eq!(ticket.event_id, 1001);
    assert_eq!(ticket.price, 1_000_000);
    assert_eq!(ticket.owner, organizer);
    assert_eq!(ticket.for_sale, false);
    assert_eq!(ticket.is_resale, false);
    assert_eq!(ticket.used, false);

    client.create_ticket(&1002, &500_000);
    let ticket2 = client.get_ticket(&1);
    assert_eq!(ticket2.id, 1);
    assert_eq!(ticket2.event_id, 1002);
}

#[test]
#[should_panic(expected = "invalid_price")]
fn test_create_ticket_negative_price_panics() {
    let (_env, client, _, _, _, _) = setup_env();
    client.create_ticket(&1001, &-100);
}

// CORREGIDO: El 'expected' string ahora coincide con el pánico real
#[test]
#[should_panic(expected = "Error(Auth, InvalidAction)")]
fn test_create_ticket_not_organizer_panics() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let organizer = Address::generate(&env);
    let platform = Address::generate(&env);
    let contract_id = env.register(SecureTicketContract, ());
    let client = SecureTicketContractClient::new(&env, &contract_id);
    let token_contract = env.register_stellar_asset_contract_v2(admin.clone());

    env.ledger().with_mut(|l| l.timestamp = 100);
    client.mock_auths(&[soroban_sdk::testutils::MockAuth {
        address: &organizer,
        invoke: &soroban_sdk::testutils::MockAuthInvoke {
            contract: &contract_id,
            fn_name: "initialize",
            args: (organizer.clone(), platform.clone(), token_contract.address(), 20_i128, 10_i128).into_val(&env),
            sub_invokes: &[],
        },
    }])
        .initialize(&organizer, &platform, &token_contract.address(), &20_i128, &10_i128);

    let random_user = Address::generate(&env);

    env.ledger().with_mut(|l| l.timestamp = 200);
    client.mock_auths(&[soroban_sdk::testutils::MockAuth {
        address: &random_user,
        invoke: &soroban_sdk::testutils::MockAuthInvoke {
            contract: &contract_id,
            fn_name: "create_ticket",
            args: (1001_u32, 1000_i128).into_val(&env),
            sub_invokes: &[],
        },
    }])
        .create_ticket(&1001, &1000); // Esto debe fallar
}

// --- Pruebas de Listado de Ticket (list_ticket) ---

#[test]
fn test_list_ticket_success() {
    let (_env, client, _, _, _, _) = setup_env();
    client.create_ticket(&101, &1000);
    client.list_ticket(&0, &1200);

    let ticket = client.get_ticket(&0);
    assert_eq!(ticket.for_sale, true);
    assert_eq!(ticket.price, 1200);
}

#[test]
#[should_panic(expected = "already_sale")]
fn test_list_ticket_already_for_sale_panics() {
    let (_env, client, _, _, _, _) = setup_env();
    // CORREGIDO: Se pasa el ID del ticket
    create_and_list_primary_ticket(&client, 0, 101, 1000);
    client.list_ticket(&0, &1500); // Debe fallar
}

#[test]
#[should_panic(expected = "ticket_used")]
fn test_list_ticket_used_panics() {
    let (env, client, token_admin, _, _, token_client) = setup_env();
    let buyer = Address::generate(&env);
    // CORREGIDO: Se pasa el ID del ticket
    let ticket_id = 0;
    sell_primary_ticket(&env, &client, &token_admin, &token_client, &buyer, ticket_id, 101, 1000);

    client.redeem_ticket(&ticket_id);
    client.list_ticket(&ticket_id, &1500); // Debe fallar
}

#[test]
#[should_panic(expected = "invalid_price")]
fn test_list_ticket_negative_price_panics() {
    let (_env, client, _, _, _, _) = setup_env();
    client.create_ticket(&101, &1000);
    client.list_ticket(&0, &-500); // Debe fallar
}

// --- Pruebas de Cancelación de Venta (cancel_sale) ---

#[test]
fn test_cancel_sale_success() {
    let (_env, client, _, _, _, _) = setup_env();
    // CORREGIDO: Se pasa el ID del ticket
    create_and_list_primary_ticket(&client, 0, 101, 1000);

    let ticket_before = client.get_ticket(&0);
    assert_eq!(ticket_before.for_sale, true);

    client.cancel_sale(&0);

    let ticket_after = client.get_ticket(&0);
    assert_eq!(ticket_after.for_sale, false);
}

#[test]
#[should_panic(expected = "not_sale")]
fn test_cancel_sale_not_for_sale_panics() {
    let (_env, client, _, _, _, _) = setup_env();
    client.create_ticket(&101, &1000); // Creado pero no listado
    client.cancel_sale(&0); // Debe fallar
}

// --- Pruebas de Compra de Ticket (buy_ticket) ---

#[test]
fn test_primary_sale_flow() {
    let (env, client, token_admin, organizer, platform, token_client) = setup_env();
    let buyer1 = Address::generate(&env);
    mint_tokens(&env, &token_admin, &token_client, &buyer1, 5_000_000);

    let price = 1_000_000;
    // CORREGIDO: Se pasa el ID del ticket
    create_and_list_primary_ticket(&client, 0, 1001, price);

    let org_bal_before = token_client.balance(&organizer);
    let plat_bal_before = token_client.balance(&platform);
    let buyer_bal_before = token_client.balance(&buyer1);

    client.buy_ticket(&0, &buyer1);

    let sold_ticket = client.get_ticket(&0);
    assert_eq!(sold_ticket.owner, buyer1);
    assert_eq!(sold_ticket.for_sale, false);
    assert_eq!(sold_ticket.is_resale, true);
    assert_eq!(sold_ticket.used, false);

    assert_eq!(token_client.balance(&buyer1), buyer_bal_before - price);
    assert_eq!(token_client.balance(&organizer), org_bal_before + price);
    assert_eq!(token_client.balance(&platform), plat_bal_before);
}

#[test]
fn test_resale_flow_with_fees() {
    let (env, client, token_admin, organizer, platform, token_client) = setup_env();

    let buyer1 = Address::generate(&env);
    let buyer2 = Address::generate(&env);

    let primary_price = 1_000_000;
    let resale_price = 2_000_000;

    mint_tokens(&env, &token_admin, &token_client, &buyer1, 5_000_000);
    mint_tokens(&env, &token_admin, &token_client, &buyer2, 5_000_000);

    // 1. Venta Primaria
    // CORREGIDO: Se pasa el ID del ticket
    let ticket_id = 0;
    sell_primary_ticket(&env, &client, &token_admin, &token_client, &buyer1, ticket_id, 101, primary_price);

    let org_bal_after_primary = token_client.balance(&organizer);
    let plat_bal_after_primary = token_client.balance(&platform);
    let buyer1_bal_after_primary = token_client.balance(&buyer1);
    let buyer2_bal_after_primary = token_client.balance(&buyer2);

    // 2. REVENTA
    client.list_ticket(&ticket_id, &resale_price);
    client.buy_ticket(&ticket_id, &buyer2);

    let resold_ticket = client.get_ticket(&0);
    assert_eq!(resold_ticket.owner, buyer2);
    assert_eq!(resold_ticket.is_resale, true);

    // Fees: Org 20% (400k), Plat 10% (200k)
    // Vendedor (Buyer1) gana 70% (1,400,000)
    assert_eq!(token_client.balance(&buyer2), buyer2_bal_after_primary - resale_price);
    assert_eq!(token_client.balance(&organizer), org_bal_after_primary + 400_000);
    assert_eq!(token_client.balance(&platform), plat_bal_after_primary + 200_000);
    assert_eq!(token_client.balance(&buyer1), buyer1_bal_after_primary + 1_400_000);
}


#[test]
#[should_panic(expected = "not_sale")]
fn test_buy_ticket_not_for_sale_panics() {
    let (env, client, token_admin, _, _, token_client) = setup_env();
    let buyer = Address::generate(&env);
    mint_tokens(&env, &token_admin, &token_client, &buyer, 1000);

    client.create_ticket(&101, &1000); // Creado pero no listado
    client.buy_ticket(&0, &buyer); // Debe fallar
}

#[test]
#[should_panic(expected = "self_buy")]
fn test_buy_ticket_self_buy_panics() {
    let (env, client, token_admin, organizer, _, token_client) = setup_env();
    mint_tokens(&env, &token_admin, &token_client, &organizer, 2000);

    // CORREGIDO: Se pasa el ID del ticket
    create_and_list_primary_ticket(&client, 0, 101, 1000);
    client.buy_ticket(&0, &organizer);
}

// CORREGIDO: El 'expected' string ahora coincide con el pánico real
#[test]
#[should_panic(expected = "Error(Contract, #10)")]
fn test_buy_ticket_insufficient_funds_panics() {
    let (env, client, _, _, _, _) = setup_env();
    let buyer = Address::generate(&env);

    // CORREGIDO: Se pasa el ID del ticket
    create_and_list_primary_ticket(&client, 0, 101, 1_000_000);
    client.buy_ticket(&0, &buyer); // Debe fallar
}

#[test]
#[should_panic(expected = "not_sale")]
fn test_buy_ticket_used_panics() {
    let (env, client, token_admin, _, _, token_client) = setup_env();
    let buyer1 = Address::generate(&env);
    let buyer2 = Address::generate(&env);
    mint_tokens(&env, &token_admin, &token_client, &buyer2, 2000);

    // CORREGIDO: Se pasa el ID del ticket
    let ticket_id = 0;
    sell_primary_ticket(&env, &client, &token_admin, &token_client, &buyer1, ticket_id, 101, 1000);

    client.list_ticket(&ticket_id, &1500);
    client.redeem_ticket(&ticket_id);

    client.buy_ticket(&ticket_id, &buyer2);
}


// --- Pruebas de Canjeo (redeem_ticket) ---

#[test]
fn test_redeem_ticket_success() {
    let (env, client, token_admin, _, _, token_client) = setup_env();
    let buyer = Address::generate(&env);
    // CORREGIDO: Se pasa el ID del ticket
    let ticket_id = 0;
    sell_primary_ticket(&env, &client, &token_admin, &token_client, &buyer, ticket_id, 101, 1000);

    let ticket_before = client.get_ticket(&ticket_id);
    assert_eq!(ticket_before.used, false);

    client.redeem_ticket(&ticket_id);

    let ticket_after = client.get_ticket(&ticket_id);
    assert_eq!(ticket_after.used, true);
    assert_eq!(ticket_after.owner, buyer);
}

#[test]
#[should_panic(expected = "already_used")]
fn test_redeem_ticket_already_used_panics() {
    let (env, client, token_admin, _, _, token_client) = setup_env();
    let buyer = Address::generate(&env);
    // CORREGIDO: Se pasa el ID del ticket
    let ticket_id = 0;
    sell_primary_ticket(&env, &client, &token_admin, &token_client, &buyer, ticket_id, 101, 1000);

    client.redeem_ticket(&ticket_id); // Primer canje (exitoso)
    client.redeem_ticket(&ticket_id); // Segundo canje (debe fallar)
}


// --- Pruebas de Vistas (Getters) ---

#[test]
#[should_panic(expected = "Ticket not found")]
fn test_get_ticket_not_found_panics() {
    let (_env, client, _, _, _, _) = setup_env();
    client.get_ticket(&999); // No existe
}

#[test]
fn test_get_owner_success() {
    let (_env, client, _, organizer, _, _) = setup_env();
    client.create_ticket(&101, &1000);
    let owner = client.get_owner(&0);
    assert_eq!(owner, organizer);
}

#[test]
fn test_get_event_tickets() {
    let (_env, client, _, _, _, _) = setup_env();

    // Evento 101
    client.create_ticket(&101, &1000); // ID 0
    client.create_ticket(&101, &1000); // ID 1

    // Evento 202
    client.create_ticket(&202, &500); // ID 2

    let tickets_101 = client.get_event_tickets(&101);
    assert_eq!(tickets_101.len(), 2);
    assert_eq!(tickets_101.get(0).unwrap().event_id, 101);

    let tickets_202 = client.get_event_tickets(&202);
    assert_eq!(tickets_202.len(), 1);

    let tickets_303 = client.get_event_tickets(&303);
    assert_eq!(tickets_303.len(), 0);
}

// CORREGIDO: Test re-escrito para usar los helpers
#[test]
fn test_get_resale_tickets() {
    let (env, client, token_admin, _, _, token_client) = setup_env();

    // Ticket 0: Venta primaria listada (NO debe aparecer)
    create_and_list_primary_ticket(&client, 0, 101, 1000);

    // Ticket 1: Vendido y listado para reventa (SÍ debe aparecer)
    let buyer1 = Address::generate(&env);
    sell_primary_ticket(&env, &client, &token_admin, &token_client, &buyer1, 1, 102, 1000); // Crea, lista, vende ticket 1
    client.list_ticket(&1, &1500); // Buyer1 lista ticket 1 para reventa

    // Ticket 2: Vendido pero NO listado (NO debe aparecer)
    let buyer2 = Address::generate(&env);
    sell_primary_ticket(&env, &client, &token_admin, &token_client, &buyer2, 2, 103, 1000); // Crea, lista, vende ticket 2
    // No se lista para reventa

    let resale_tickets = client.get_resale_tickets();

    assert_eq!(resale_tickets.len(), 1);
    assert_eq!(resale_tickets.get(0).unwrap().id, 1);
    assert_eq!(resale_tickets.get(0).unwrap().is_resale, true);
    assert_eq!(resale_tickets.get(0).unwrap().for_sale, true);
}